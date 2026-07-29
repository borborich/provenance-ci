#!/usr/bin/env python3
"""Validate the dependency-free GitHub Pages site."""

from __future__ import annotations

from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import unquote, urlparse
import sys
import xml.etree.ElementTree as ET


ROOT = Path(__file__).resolve().parents[1]
SITE = ROOT / "site"
PUBLIC_BASE = "https://borborich.github.io/provenance-ci/"


class PageParser(HTMLParser):
    def __init__(self) -> None:
        super().__init__()
        self.canonicals: list[str] = []
        self.descriptions: list[str] = []
        self.hrefs: list[str] = []
        self.in_title = False
        self.title = ""
        self.html_lang: str | None = None

    def handle_starttag(
        self, tag: str, attrs: list[tuple[str, str | None]]
    ) -> None:
        values = dict(attrs)
        if tag == "html":
            self.html_lang = values.get("lang")
        elif tag == "title":
            self.in_title = True
        elif tag == "link" and values.get("rel") == "canonical":
            href = values.get("href")
            if href:
                self.canonicals.append(href)
        elif tag == "meta" and values.get("name") == "description":
            content = values.get("content")
            if content:
                self.descriptions.append(content)
        elif tag == "a":
            href = values.get("href")
            if href:
                self.hrefs.append(href)

    def handle_endtag(self, tag: str) -> None:
        if tag == "title":
            self.in_title = False

    def handle_data(self, data: str) -> None:
        if self.in_title:
            self.title += data


def public_url(path: Path) -> str:
    relative = path.relative_to(SITE)
    if relative.name == "index.html":
        route = relative.parent.as_posix()
        return PUBLIC_BASE if route == "." else f"{PUBLIC_BASE}{route}/"
    return f"{PUBLIC_BASE}{relative.as_posix()}"


def resolve_internal(page: Path, href: str) -> Path | None:
    parsed = urlparse(href)
    if parsed.scheme or parsed.netloc or href.startswith(("#", "mailto:")):
        return None

    target = unquote(parsed.path)
    if not target:
        return None
    if target.startswith("/provenance-ci/"):
        candidate = SITE / target.removeprefix("/provenance-ci/")
    elif target == "/provenance-ci":
        candidate = SITE
    elif target.startswith("/"):
        return None
    else:
        candidate = page.parent / target

    if candidate.is_dir() or target.endswith("/"):
        candidate = candidate / "index.html"
    return candidate.resolve()


def main() -> int:
    errors: list[str] = []
    pages = sorted(SITE.rglob("*.html"))

    if not pages:
        errors.append("site contains no HTML pages")

    expected_urls: set[str] = set()
    for page in pages:
        parser = PageParser()
        parser.feed(page.read_text(encoding="utf-8"))
        expected = public_url(page)
        expected_urls.add(expected)

        if parser.html_lang != "en":
            errors.append(f"{page}: expected html lang=en")
        if not parser.title.strip():
            errors.append(f"{page}: missing title")
        if parser.canonicals != [expected]:
            errors.append(
                f"{page}: canonical must be exactly {expected!r}, got "
                f"{parser.canonicals!r}"
            )
        if len(parser.descriptions) != 1:
            errors.append(f"{page}: expected one meta description")

        for href in parser.hrefs:
            target = resolve_internal(page, href)
            if target is not None and not target.exists():
                errors.append(f"{page}: broken internal link {href!r}")

    sitemap = SITE / "sitemap.xml"
    try:
        tree = ET.parse(sitemap)
        namespace = {"s": "http://www.sitemaps.org/schemas/sitemap/0.9"}
        sitemap_urls = {
            element.text
            for element in tree.findall("s:url/s:loc", namespace)
            if element.text
        }
        indexable_urls = {
            url for url in expected_urls if not url.endswith("404.html")
        }
        if sitemap_urls != indexable_urls:
            errors.append(
                "sitemap URLs differ from indexable HTML pages: "
                f"expected {sorted(indexable_urls)!r}, got {sorted(sitemap_urls)!r}"
            )
    except (FileNotFoundError, ET.ParseError) as exc:
        errors.append(f"invalid sitemap.xml: {exc}")

    robots = SITE / "robots.txt"
    if not robots.exists():
        errors.append("missing robots.txt")
    elif f"Sitemap: {PUBLIC_BASE}sitemap.xml" not in robots.read_text(
        encoding="utf-8"
    ):
        errors.append("robots.txt does not reference the canonical sitemap")

    llms = SITE / "llms.txt"
    if not llms.exists():
        errors.append("missing llms.txt")

    if errors:
        print("Static site validation failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    print(f"Static site validation passed for {len(pages)} HTML pages.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
