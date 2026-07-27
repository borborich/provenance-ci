use std::{
    collections::BTreeMap,
    fs,
    io::Read,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, ToSocketAddrs},
    path::Path,
    time::Duration,
};

use anyhow::{anyhow, bail, Context, Result};
use reqwest::{
    blocking::Client,
    header::{HeaderMap, ACCEPT, CONTENT_TYPE, LOCATION, USER_AGENT},
    redirect::Policy,
};
use url::Url;

use crate::{config::CheckpointConfig, model::SourceEvidence};

pub const MAX_ASSET_BYTES: u64 = 25 * 1024 * 1024;
const MAX_REDIRECTS: usize = 5;

#[derive(Debug)]
pub struct FetchedAsset {
    pub bytes: Vec<u8>,
    pub media_type: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub source: SourceEvidence,
    pub supported: bool,
}

pub fn source_skeleton(checkpoint: &CheckpointConfig) -> SourceEvidence {
    if let Some(path) = &checkpoint.path {
        SourceEvidence {
            kind: "path".into(),
            value: path.clone(),
            final_url: None,
            http_status: None,
            http_headers: BTreeMap::new(),
        }
    } else {
        let value = checkpoint
            .url
            .as_deref()
            .map(redact_url)
            .unwrap_or_else(|| "<missing>".into());
        SourceEvidence {
            kind: "url".into(),
            value,
            final_url: None,
            http_status: None,
            http_headers: BTreeMap::new(),
        }
    }
}

pub fn acquire(checkpoint: &CheckpointConfig, config_dir: &Path) -> Result<FetchedAsset> {
    match (&checkpoint.path, &checkpoint.url) {
        (Some(path), None) => acquire_path(path, config_dir),
        (None, Some(url)) => acquire_url(url),
        _ => bail!("checkpoint must set exactly one source"),
    }
}

fn acquire_path(value: &str, config_dir: &Path) -> Result<FetchedAsset> {
    let path = config_dir.join(value);
    let metadata =
        fs::metadata(&path).with_context(|| format!("cannot stat {}", path.display()))?;
    if metadata.len() > MAX_ASSET_BYTES {
        bail!(
            "asset exceeds {} byte limit: {}",
            MAX_ASSET_BYTES,
            path.display()
        );
    }
    let bytes = fs::read(&path).with_context(|| format!("cannot read {}", path.display()))?;
    let (media_type, supported) = sniff_media_type(&bytes);
    let dimensions = if supported {
        jpeg_dimensions(&bytes)
    } else {
        None
    };
    Ok(FetchedAsset {
        bytes,
        media_type: media_type.into(),
        width: dimensions.map(|d| d.0),
        height: dimensions.map(|d| d.1),
        source: SourceEvidence {
            kind: "path".into(),
            value: value.into(),
            final_url: None,
            http_status: None,
            http_headers: BTreeMap::new(),
        },
        supported,
    })
}

fn acquire_url(value: &str) -> Result<FetchedAsset> {
    let mut current = Url::parse(value).context("invalid checkpoint URL")?;
    let mut redirect_count = 0;

    loop {
        validate_public_https_url(&current)?;
        let host = current
            .host_str()
            .ok_or_else(|| anyhow!("URL must contain a hostname"))?;
        let port = current.port_or_known_default().unwrap_or(443);
        let addresses = resolve_public(host, port)?;
        let pinned = addresses
            .first()
            .copied()
            .ok_or_else(|| anyhow!("hostname resolved to no addresses"))?;

        let client = Client::builder()
            .no_proxy()
            .redirect(Policy::none())
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(20))
            .resolve(host, pinned)
            .build()
            .context("failed to build HTTP client")?;
        let response = client
            .get(current.clone())
            .header(
                USER_AGENT,
                concat!("provenance-ci/", env!("CARGO_PKG_VERSION")),
            )
            .header(ACCEPT, "image/jpeg")
            .send()
            .context("HTTPS fetch failed")?;

        if response.status().is_redirection() {
            if redirect_count >= MAX_REDIRECTS {
                bail!("redirect limit exceeded");
            }
            let location = response
                .headers()
                .get(LOCATION)
                .ok_or_else(|| anyhow!("redirect response has no Location header"))?
                .to_str()
                .context("redirect Location is not valid text")?;
            current = current.join(location).context("invalid redirect URL")?;
            redirect_count += 1;
            continue;
        }

        if !response.status().is_success() {
            bail!("HTTP request returned status {}", response.status());
        }
        if let Some(length) = response.content_length() {
            if length > MAX_ASSET_BYTES {
                bail!("HTTP asset exceeds {MAX_ASSET_BYTES} byte limit");
            }
        }
        let status = response.status().as_u16();
        let headers = selected_headers(response.headers());
        let declared_type = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.split(';').next())
            .unwrap_or("")
            .trim()
            .to_ascii_lowercase();

        let mut bytes = Vec::new();
        response
            .take(MAX_ASSET_BYTES + 1)
            .read_to_end(&mut bytes)
            .context("failed while streaming HTTP response")?;
        if bytes.len() as u64 > MAX_ASSET_BYTES {
            bail!("HTTP asset exceeds {MAX_ASSET_BYTES} byte limit");
        }
        let (sniffed_type, supported) = sniff_media_type(&bytes);
        if declared_type == "text/html" || sniffed_type == "text/html" {
            bail!("HTML response is not accepted as an image");
        }
        let dimensions = if supported {
            jpeg_dimensions(&bytes)
        } else {
            None
        };

        return Ok(FetchedAsset {
            bytes,
            media_type: sniffed_type.into(),
            width: dimensions.map(|d| d.0),
            height: dimensions.map(|d| d.1),
            source: SourceEvidence {
                kind: "url".into(),
                value: redact_url(value),
                final_url: Some(redact_url(current.as_str())),
                http_status: Some(status),
                http_headers: headers,
            },
            supported,
        });
    }
}

fn selected_headers(headers: &HeaderMap) -> BTreeMap<String, String> {
    const NAMES: &[&str] = &[
        "content-type",
        "content-length",
        "content-encoding",
        "etag",
        "last-modified",
        "cache-control",
        "vary",
        "accept-ranges",
    ];
    NAMES
        .iter()
        .filter_map(|name| {
            headers.get(*name).and_then(|value| {
                value
                    .to_str()
                    .ok()
                    .map(|value| ((*name).to_string(), value.to_string()))
            })
        })
        .collect()
}

fn resolve_public(host: &str, port: u16) -> Result<Vec<SocketAddr>> {
    let mut addresses: Vec<_> = (host, port)
        .to_socket_addrs()
        .with_context(|| format!("failed to resolve hostname {host}"))?
        .collect();
    addresses.sort_unstable();
    addresses.dedup();
    if addresses.is_empty() {
        bail!("hostname resolved to no addresses");
    }
    if let Some(address) = addresses.iter().find(|address| !is_public_ip(address.ip())) {
        bail!(
            "hostname resolves to a non-public or reserved address: {}",
            address.ip()
        );
    }
    Ok(addresses)
}

fn validate_public_https_url(url: &Url) -> Result<()> {
    if url.scheme() != "https" {
        bail!("only public HTTPS URLs are allowed");
    }
    if !url.username().is_empty() || url.password().is_some() {
        bail!("URL credentials are not allowed");
    }
    if url.port().is_some_and(|port| port != 443) {
        bail!("only HTTPS port 443 is allowed");
    }
    match url.host() {
        Some(url::Host::Ipv4(ip)) if !is_public_ip(IpAddr::V4(ip)) => {
            bail!("non-public or reserved IP address is not allowed")
        }
        Some(url::Host::Ipv6(ip)) if !is_public_ip(IpAddr::V6(ip)) => {
            bail!("non-public or reserved IP address is not allowed")
        }
        Some(_) => Ok(()),
        None => bail!("URL must contain a hostname"),
    }
}

pub fn is_public_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ip) => is_public_ipv4(ip),
        IpAddr::V6(ip) => {
            if let Some(mapped) = ip.to_ipv4_mapped() {
                return is_public_ipv4(mapped);
            }
            is_public_ipv6(ip)
        }
    }
}

fn is_public_ipv4(ip: Ipv4Addr) -> bool {
    let value = u32::from(ip);
    let in_net = |base: [u8; 4], prefix: u32| {
        let mask = if prefix == 0 {
            0
        } else {
            u32::MAX << (32 - prefix)
        };
        value & mask == u32::from(Ipv4Addr::from(base)) & mask
    };
    ![
        ([0, 0, 0, 0], 8),
        ([10, 0, 0, 0], 8),
        ([100, 64, 0, 0], 10),
        ([127, 0, 0, 0], 8),
        ([169, 254, 0, 0], 16),
        ([172, 16, 0, 0], 12),
        ([192, 0, 0, 0], 24),
        ([192, 0, 2, 0], 24),
        ([192, 168, 0, 0], 16),
        ([198, 18, 0, 0], 15),
        ([198, 51, 100, 0], 24),
        ([203, 0, 113, 0], 24),
        ([224, 0, 0, 0], 4),
        ([240, 0, 0, 0], 4),
    ]
    .iter()
    .any(|(base, prefix)| in_net(*base, *prefix))
}

fn is_public_ipv6(ip: Ipv6Addr) -> bool {
    let octets = ip.octets();
    !(ip.is_unspecified()
        || ip.is_loopback()
        || ip.is_multicast()
        || (octets[0] & 0xfe) == 0xfc
        || (octets[0] == 0xfe && octets[1] & 0xc0 == 0x80)
        || (octets[0] == 0x20 && octets[1] == 0x01 && octets[2] == 0x0d && octets[3] == 0xb8))
}

fn sniff_media_type(bytes: &[u8]) -> (&'static str, bool) {
    if bytes.starts_with(&[0xff, 0xd8, 0xff]) {
        ("image/jpeg", true)
    } else if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        ("image/png", false)
    } else if bytes
        .iter()
        .take(256)
        .copied()
        .filter(|byte| !byte.is_ascii_whitespace())
        .take(1)
        .any(|byte| byte == b'<')
    {
        ("text/html", false)
    } else {
        ("application/octet-stream", false)
    }
}

fn jpeg_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if !bytes.starts_with(&[0xff, 0xd8]) {
        return None;
    }
    let mut index = 2;
    while index + 9 < bytes.len() {
        if bytes[index] != 0xff {
            index += 1;
            continue;
        }
        while index < bytes.len() && bytes[index] == 0xff {
            index += 1;
        }
        let marker = *bytes.get(index)?;
        index += 1;
        if marker == 0xd8 || marker == 0xd9 || marker == 0x01 {
            continue;
        }
        let length = u16::from_be_bytes([*bytes.get(index)?, *bytes.get(index + 1)?]) as usize;
        if length < 2 || index + length > bytes.len() {
            return None;
        }
        if matches!(
            marker,
            0xc0 | 0xc1
                | 0xc2
                | 0xc3
                | 0xc5
                | 0xc6
                | 0xc7
                | 0xc9
                | 0xca
                | 0xcb
                | 0xcd
                | 0xce
                | 0xcf
        ) {
            let height =
                u16::from_be_bytes([*bytes.get(index + 3)?, *bytes.get(index + 4)?]) as u32;
            let width = u16::from_be_bytes([*bytes.get(index + 5)?, *bytes.get(index + 6)?]) as u32;
            return Some((width, height));
        }
        index += length;
    }
    None
}

pub fn redact_url(value: &str) -> String {
    Url::parse(value).map_or_else(
        |_| "<invalid-url>".into(),
        |mut url| {
            url.set_query(None);
            url.set_fragment(None);
            url.to_string()
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_special_networks_and_mapped_ipv4() {
        for ip in [
            "127.0.0.1",
            "10.0.0.1",
            "169.254.169.254",
            "192.0.2.1",
            "::1",
            "fc00::1",
            "fe80::1",
            "2001:db8::1",
            "::ffff:127.0.0.1",
        ] {
            assert!(!is_public_ip(ip.parse().expect("valid test IP")), "{ip}");
        }
        assert!(is_public_ip("8.8.8.8".parse().expect("valid test IP")));
        assert!(is_public_ip(
            "2606:4700:4700::1111".parse().expect("valid test IP")
        ));
    }

    #[test]
    fn redacts_url_queries_and_fragments() {
        assert_eq!(
            redact_url("https://example.com/a.jpg?token=secret#fragment"),
            "https://example.com/a.jpg"
        );
    }

    #[test]
    fn parses_jpeg_dimensions() {
        let jpeg = [
            0xff, 0xd8, 0xff, 0xc0, 0x00, 0x11, 0x08, 0x00, 0x10, 0x00, 0x20, 0x03, 0x01, 0x11,
            0x00, 0x02, 0x11, 0x00, 0x03, 0x11, 0x00, 0xff, 0xd9,
        ];
        assert_eq!(jpeg_dimensions(&jpeg), Some((32, 16)));
    }

    #[test]
    fn recognizes_png_as_explicitly_unsupported() {
        assert_eq!(
            sniff_media_type(b"\x89PNG\r\n\x1a\nfixture"),
            ("image/png", false)
        );
    }
}
