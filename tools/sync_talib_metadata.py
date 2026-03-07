#!/usr/bin/env python3
"""Refresh pinned TA-Lib metadata snapshots used by PalmScript tooling.

This script intentionally lives outside the runtime path. It fetches the
repository-pinned upstream metadata and writes raw snapshots into
`vendor/talib/` so CI and local review can diff the exact source inputs.
"""

from __future__ import annotations

import pathlib
import urllib.request


COMMIT = "1bdf54384036852952b8b4cb97c09359ae407bd0"
FILES = {
    "ta_func_api.xml": f"https://raw.githubusercontent.com/TA-Lib/ta-lib/{COMMIT}/ta_func_api.xml",
    "ta_func_list.txt": f"https://raw.githubusercontent.com/TA-Lib/ta-lib/{COMMIT}/ta_func_list.txt",
    "LICENSE": f"https://raw.githubusercontent.com/TA-Lib/ta-lib/{COMMIT}/LICENSE",
}


def main() -> None:
    root = pathlib.Path(__file__).resolve().parent.parent
    out_dir = root / "vendor" / "talib"
    out_dir.mkdir(parents=True, exist_ok=True)
    for name, url in FILES.items():
        with urllib.request.urlopen(url) as response:
            (out_dir / name).write_bytes(response.read())
    print(f"synced {len(FILES)} files from TA-Lib {COMMIT} into {out_dir}")


if __name__ == "__main__":
    main()
