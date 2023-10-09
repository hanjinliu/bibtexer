# bibtexer

A toolkit for management of references in BibTeX format for scientific paper writing.

# Examples

```python
from bibtexer import convert_text

text = """
@article{
    title={My great paper},
    author={Liu, Hanjin and AAA, XXX and BBB, YYY and CCC, ZZZ},
    journal={Nature},
    volume={999},
    pages={999999},
    year={2023}
}
"""

convert_text(text, format="APA")
# Out: "Liu, H., AAA, X., BBB, Y. & CCC, Z. (2023). My great paper. Nature, 999, 999999."

convert_text(text, format="simple")
# Out: "Liu et al., 2023"
```
