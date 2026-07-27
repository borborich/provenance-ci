#!/usr/bin/env python3
"""Create a byte-stable gzip-compressed tar archive for one directory."""

import gzip
import os
import pathlib
import sys
import tarfile


def entries(root: pathlib.Path):
    yield root
    for directory, dirnames, filenames in os.walk(root):
        dirnames.sort()
        filenames.sort()
        base = pathlib.Path(directory)
        for dirname in dirnames:
            yield base / dirname
        for filename in filenames:
            yield base / filename


def normalized_info(
    tar: tarfile.TarFile, path: pathlib.Path, root: pathlib.Path
) -> tarfile.TarInfo:
    relative = path.relative_to(root.parent)
    info = tar.gettarinfo(str(path), arcname=str(relative))
    info.uid = 0
    info.gid = 0
    info.uname = ""
    info.gname = ""
    info.mtime = 0
    if info.isdir():
        info.mode = 0o755
    elif info.isfile():
        info.mode = 0o755 if path.stat().st_mode & 0o111 else 0o644
    return info


def main() -> int:
    if len(sys.argv) != 3:
        print("usage: deterministic-tar.py DIRECTORY ARCHIVE", file=sys.stderr)
        return 2

    root = pathlib.Path(sys.argv[1]).resolve()
    archive = pathlib.Path(sys.argv[2]).resolve()
    if not root.is_dir():
        print(f"not a directory: {root}", file=sys.stderr)
        return 2
    archive.parent.mkdir(parents=True, exist_ok=True)

    with archive.open("wb") as raw:
        with gzip.GzipFile(
            filename="", mode="wb", fileobj=raw, compresslevel=9, mtime=0
        ) as compressed:
            with tarfile.open(
                fileobj=compressed, mode="w", format=tarfile.GNU_FORMAT
            ) as tar:
                for path in entries(root):
                    info = normalized_info(tar, path, root)
                    if info.isfile():
                        with path.open("rb") as source:
                            tar.addfile(info, source)
                    else:
                        tar.addfile(info)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
