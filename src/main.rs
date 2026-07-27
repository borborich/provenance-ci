use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    process::ExitCode,
};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use clap::{Parser, ValueEnum};
use provenance_ci::{report, Config, RunOptions};

#[derive(Debug, Parser)]
#[command(
    name = "provenance-ci",
    version,
    about = "CI regression and continuity testing for Content Credentials"
)]
struct Cli {
    /// Versioned YAML configuration file.
    #[arg(short, long, default_value = "provenance-ci.yml")]
    config: PathBuf,
    /// JSON result path. The file is always written.
    #[arg(short, long, default_value = "provenance-ci-result.json")]
    output: PathBuf,
    /// Optional Markdown report path (use $GITHUB_STEP_SUMMARY in Actions).
    #[arg(long)]
    markdown: Option<PathBuf>,
    /// Terminal output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Human)]
    format: OutputFormat,
    /// Emit GitHub workflow commands to stdout.
    #[arg(long)]
    github_annotations: bool,
    /// Fixed RFC3339 timestamp for reproducible tests.
    #[arg(long, hide = true)]
    checked_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum OutputFormat {
    Human,
    Json,
}

fn main() -> ExitCode {
    match execute() {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            eprintln!("provenance-ci: {error:#}");
            ExitCode::from(3)
        }
    }
}

fn execute() -> Result<u8> {
    let cli = Cli::parse();
    let config = Config::from_path(&cli.config)?;
    let result = provenance_ci::run(
        &config,
        &RunOptions {
            config_path: cli.config.clone(),
            checked_at: cli.checked_at,
        },
    )?;
    let json = serde_json::to_string_pretty(&result).context("failed to serialize JSON result")?;
    fs::write(&cli.output, format!("{json}\n"))
        .with_context(|| format!("failed to write {}", cli.output.display()))?;
    if let Some(path) = &cli.markdown {
        fs::write(path, report::markdown(&result))
            .with_context(|| format!("failed to write {}", path.display()))?;
    }

    let mut stdout = io::stdout().lock();
    match cli.format {
        OutputFormat::Human => stdout.write_all(report::human(&result).as_bytes())?,
        OutputFormat::Json => {
            stdout.write_all(json.as_bytes())?;
            stdout.write_all(b"\n")?;
        }
    }
    if cli.github_annotations {
        stdout.write_all(report::github_annotations(&result).as_bytes())?;
    }
    Ok(u8::try_from(result.exit_code).unwrap_or(3))
}
