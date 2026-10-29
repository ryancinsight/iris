"""Reproduce reference RGB fixtures from one verified Matplotlib wheel.

Use the reference-only environment recorded in the verification notes. This
does not import Iris or derive reference colors from Iris's control points.
"""

from __future__ import annotations

import argparse
import hashlib
from importlib import metadata
import json
import math
import os
from pathlib import Path
import sysconfig
import tempfile


RELEASE = "3.11.2"
COMMIT = "d3ca9172e392048ec242b90e8423e88d1a3000b2"
NUMPY = "2.4.6"
PALETTES = ("bone", "jet", "plasma", "viridis", "inferno", "magma", "turbo")
SOURCE_HASHES = {
    "_cm.py": "44dc9e0f2801d1a04a4abc9f88bbea672ebcf2e1fd9efc4fac49e8fea9ed4f96",
    "_cm_listed.py": "ddad3698f5129ceb1792a445371286c08bc9080298e657b3054aea19c9659ef9",
    "colors.py": "ef4173f9e8310bc75714e32c3855719c27916c6c651b5c94e39f00163842cf04",
}
LICENSE_HASH = "5a1a81ea301728c8bba2933da832c0cd62229daf20893a024ab3d53244468dbc"
ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "tests/fixtures/palette_reference"


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def verify_fixtures() -> None:
    """Check committed provenance and bytes without importing reference tools."""
    manifest = json.loads((OUTPUT / "manifest.json").read_text())
    expected = {
        "schema": 1, "project": "matplotlib/matplotlib", "release": RELEASE,
        "commit": COMMIT, "source_sha256": SOURCE_HASHES,
        "license_sha256": LICENSE_HASH, "lut_length": 256, "grid_steps": 1024,
    }
    for name, value in expected.items():
        if manifest.get(name) != value:
            raise ValueError(f"reference provenance drift: {name}")
    if set(manifest["palettes"]) != set(PALETTES):
        raise ValueError("reference palette set differs")
    if digest((OUTPUT / "MATPLOTLIB-LICENSE").read_bytes()) != LICENSE_HASH:
        raise ValueError("reference license drift")
    for name in PALETTES:
        record = manifest["palettes"][name]
        if record["file"] != name + ".json":
            raise ValueError(f"reference filename differs: {name}")
        data = (OUTPUT / record["file"]).read_bytes()
        if digest(data) != record["sha256"]:
            raise ValueError(f"reference fixture drift: {name}")
        rows = json.loads(data)
        if len(rows) != 256 or any(
            len(row) != 3 or any(not math.isfinite(v) or not 0 <= v <= 1 for v in row)
            for row in rows
        ):
            raise ValueError(f"invalid reference RGB dimensions or values: {name}")
    print("verified seven pinned reference fixtures, provenance and license")


def verified_library():
    if metadata.version("matplotlib") != RELEASE or metadata.version("numpy") != NUMPY:
        raise ValueError(f"requires matplotlib=={RELEASE} and numpy=={NUMPY}")
    directory = Path(sysconfig.get_path("purelib")) / "matplotlib"
    for name, expected in SOURCE_HASHES.items():
        if digest((directory / name).read_bytes()) != expected:
            raise ValueError(f"wheel {name} differs from pinned upstream commit {COMMIT}")
    distribution = metadata.distribution("matplotlib")
    notices = [
        distribution.locate_file(path).read_bytes()
        for path in distribution.files or []
        if path.name == "LICENSE"
    ]
    license_text = (OUTPUT / "MATPLOTLIB-LICENSE").read_bytes()
    if digest(license_text) != LICENSE_HASH:
        raise ValueError("reference notice differs from the pinned upstream license")
    # The wheel concatenates upstream and bundled-library notices. Preserve
    # the exact relevant upstream notice, and prove it is present in the wheel.
    if not any(license_text in data for data in notices):
        raise ValueError("wheel lacks the exact pinned Matplotlib license text")
    return license_text


def generate(license_text: bytes) -> dict[str, bytes]:
    # Source hashes are checked before importing the released reference tool.
    import matplotlib
    import numpy as np

    documents = {"MATPLOTLIB-LICENSE": license_text}
    provenance = {
        "schema": 1,
        "project": "matplotlib/matplotlib",
        "release": RELEASE,
        "commit": COMMIT,
        "source_sha256": SOURCE_HASHES,
        "license_sha256": LICENSE_HASH,
        "numpy": NUMPY,
        "lut_length": 256,
        "grid_steps": 1024,
        "sampling": "floor(256*t) for 0<=t<1; t=1 selects row255",
        "channels": "normalized sRGB-encoded RGB; reference alpha is always1",
        "generation": "sample released default colormaps; omit constant alpha; reformat RGB as JSON",
        "palettes": {},
    }
    for name in PALETTES:
        palette = matplotlib.colormaps[name]
        if palette.N != 256 or palette.name != name:
            raise ValueError(f"unexpected reference palette identity: {name}")
        rows = [[float(channel) for channel in palette(index / 255.0)[:3]]
                for index in range(256)]
        if not all(np.isfinite(row).all() and all(0 <= x <= 1 for x in row) for row in rows):
            raise ValueError(f"invalid reference RGB values: {name}")
        # All comparison inputs are exact binary fractions. Verify the lookup
        # convention against the real API instead of approximating its parser.
        for step in range(1025):
            actual = palette(step / 1024.0)
            index = min(step // 4, 255)
            if list(actual[:3]) != rows[index] or actual[3] != 1:
                raise ValueError(f"reference sampling convention differs: {name}, {step}")
        text = "[\n" + ",\n".join("  " + json.dumps(row) for row in rows) + "\n]\n"
        data = text.encode()
        filename = name + ".json"
        documents[filename] = data
        provenance["palettes"][name] = {
            "file": filename,
            "source": "_cm.py" if name in ("bone", "jet") else "_cm_listed.py",
            "kind": type(palette).__name__,
            "sha256": digest(data),
        }
    documents["manifest.json"] = (json.dumps(provenance, indent=2) + "\n").encode()
    return documents


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--check", action="store_true", help="compare only; never rewrite fixtures")
    mode.add_argument("--verify-fixtures", action="store_true", help="verify committed bytes without Matplotlib")
    args = parser.parse_args()
    if args.verify_fixtures:
        verify_fixtures()
        return
    license_text = verified_library()
    with tempfile.TemporaryDirectory(prefix="iris-mpl-config-") as directory:
        os.environ["MPLCONFIGDIR"] = directory
        os.environ["MPLBACKEND"] = "Agg"
        documents = generate(license_text)
    if args.check:
        mismatched = [name for name, data in documents.items()
                      if not (OUTPUT / name).is_file() or (OUTPUT / name).read_bytes() != data]
        if mismatched:
            raise SystemExit("reference fixture drift: " + ", ".join(mismatched))
        print(f"verified {len(PALETTES)} pinned palettes and 256-entry sampling on 1025 coordinates")
        return
    OUTPUT.mkdir(parents=True, exist_ok=True)
    for name, data in documents.items():
        (OUTPUT / name).write_bytes(data)
    print(f"generated {len(PALETTES)} pinned palette fixtures")


if __name__ == "__main__":
    main()
