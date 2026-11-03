"""Reproduce the release archive from this public source (Python 3.11+)."""
import hashlib
import json
import tomllib
import zipfile
from pathlib import Path
STAMP = (2026, 1, 1, 0, 0, 0)

def manifest(compatibility, bundle):
    pinned = bundle["compatibility"]["horizon_revision"]
    if pinned != compatibility["horizon_revision"]:
        raise SystemExit(f"horizon-mixin.toml pins {pinned}, compatibility.json {compatibility['horizon_revision']}")
    return {
        "schema": 1,
        "id": bundle["mod"]["id"],
        "version": bundle["mod"]["version"],
        "game": compatibility["game"],
        "horizon": pinned,
        "mixins": compatibility["horizon_mixins_revision"],
    }


def package(mod, out):
    """The mod as `out`/<id>-<version>.hnmod, with the checkout's line endings gone."""
    compatibility = json.loads((mod / "compatibility.json").read_text(encoding="utf-8"))
    source = next(p for p in (mod / "mixins").iterdir() if (p / "horizon-mixin.toml").is_file())
    bundle = tomllib.loads((source / "horizon-mixin.toml").read_text(encoding="utf-8"))
    native = manifest(compatibility, bundle)
    files = {"horizon-native.json": (json.dumps(native, indent=2, sort_keys=True) + "\n").encode()}
    for path in sorted(source.iterdir()):
        if path.suffix in (".rs", ".toml"):
            files[f"mixins/{native['id']}/{path.name}"] = path.read_bytes().replace(b"\r\n", b"\n")
    out.mkdir(parents=True, exist_ok=True)
    target = out / f"{native['id']}-{native['version']}.hnmod"
    with zipfile.ZipFile(target, "w", compression=zipfile.ZIP_DEFLATED) as archive:
        for name, content in sorted(files.items()):
            info = zipfile.ZipInfo(name, STAMP)
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o100644 << 16
            archive.writestr(info, content)
    return target


if __name__ == "__main__":
    root = Path(__file__).resolve().parents[1]
    archive = package(root, root / "target")
    print(archive)
    print(hashlib.sha256(archive.read_bytes()).hexdigest())
