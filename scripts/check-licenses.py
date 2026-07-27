#!/usr/bin/env python3
"""Create a lockfile-based license inventory and reject known forbidden terms."""

import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
FORBIDDEN = ("AGPL", "SSPL", "BUSL", "COMMONS-CLAUSE")


def main() -> int:
    completed = subprocess.run(
        [
            "cargo",
            "metadata",
            "--format-version",
            "1",
            "--locked",
            "--manifest-path",
            str(ROOT / "Cargo.toml"),
        ],
        capture_output=True,
        text=True,
    )
    if completed.returncode != 0:
        print(completed.stderr, file=sys.stderr, end="")
        return completed.returncode
    metadata = json.loads(completed.stdout)
    packages = []
    errors = []
    for package in sorted(
        metadata["packages"], key=lambda item: (item["name"], item["version"])
    ):
        if package["name"] == "provenance-ci":
            continue
        license_expression = package.get("license")
        license_file = package.get("license_file")
        display = license_expression or (
            f"LicenseRef-file:{license_file}" if license_file else None
        )
        if not display:
            errors.append(
                f"{package['name']} {package['version']}: no license metadata"
            )
        upper = (display or "").upper()
        if any(term in upper for term in FORBIDDEN):
            errors.append(
                f"{package['name']} {package['version']}: forbidden license {display}"
            )
        packages.append(
            {
                "name": package["name"],
                "version": package["version"],
                "license": display,
                "repository": package.get("repository"),
                "source": package.get("source"),
            }
        )

    output = {
        "schemaVersion": 1,
        "generatedFrom": "Cargo.lock",
        "packageCount": len(packages),
        "packages": packages,
    }
    dist = ROOT / "dist"
    dist.mkdir(exist_ok=True)
    path = dist / "dependency-licenses.json"
    path.write_text(json.dumps(output, indent=2, sort_keys=True) + "\n")

    if errors:
        for error in errors:
            print(error, file=sys.stderr)
        return 1
    print(f"License inventory passed for {len(packages)} packages: {path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
