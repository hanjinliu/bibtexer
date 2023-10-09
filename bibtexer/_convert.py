from __future__ import annotations

from pathlib import Path
from ._bibtexer_rust import convert_bibtex_vectorized

FORMATS_DIR = Path(__file__).parent / "formats"

def _get_format_text(name: str) -> str:
    return FORMATS_DIR.joinpath(f"{name}.json").read_text()

def convert_text(text: str, format: str) -> list[str]:
    out = convert_bibtex_vectorized(text, _get_format_text(format))
    return out

def convert_file(path: str | Path, format: str) -> str:
    return convert_text(Path(path).read_text(), format)
