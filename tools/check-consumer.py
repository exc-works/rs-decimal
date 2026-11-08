"""Compile a fresh edition-2024 consumer without borrowing the workspace lockfile."""
import json
import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def main():
    # A private temporary crate prevents workspace feature unification and lock reuse.
    with tempfile.TemporaryDirectory(prefix="rs-decimal-consumer-") as directory:
        consumer = Path(directory)
        (consumer / "src").mkdir()
        # JSON string escaping is also valid for this TOML basic string.
        (consumer / "Cargo.toml").write_text(
            '[package]\nname = "decimal-consumer-check"\nversion = "0.0.0"\n'
            'edition = "2024"\nrust-version = "1.85"\n'
            '[dependencies]\nrs-decimal = { path = '
            + json.dumps(ROOT.as_posix())
            + ', features = ["serde", "json", "config", "validation", "postgres", "yaml", "xml", "bson"] }\n',
            encoding="utf-8",
        )
        (consumer / "src/main.rs").write_text(
            'use rs_decimal::{Decimal, DecimalError, ErrorKind, RoundingMode};\n'
            'fn main() -> Result<(), DecimalError> {\n'
            'let x: Decimal = "1.25".parse()?;\n'
            'assert_eq!(x.rescale(1, RoundingMode::HalfEven)?.string(), "1.2");\n'
            'assert!(x.sqrt_with_prec(-1).unwrap_err().matches(ErrorKind::InvalidPrecision));\n'
            'assert!(x.quo_down(&Decimal::new(0)).unwrap_err().matches(ErrorKind::DivideByZero));\n'
            'Ok(())\n'
            '}\n', encoding="utf-8",
        )
        environment = dict(os.environ)
        environment.setdefault("CARGO_TARGET_DIR", str(ROOT / "target/consumer"))
        subprocess.run(["cargo", "run", "--manifest-path", str(consumer / "Cargo.toml")],
                       env=environment, check=True)


if __name__ == "__main__":
    main()
