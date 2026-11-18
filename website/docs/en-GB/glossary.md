---
locale: en-GB
source_revision: 8aa8846dbb08293336b9672dd606ae4d60eef2ecb4ca240c302f5b6b1f047935
---
# Glossary

| Term | Meaning |
| --- | --- |
| coefficient | Arbitrary-size signed integer containing the digits. |
| scale | Nonnegative count of decimal places: value = coefficient × 10^−scale. |
| precision | Usually decimal places in method arguments; significant-figure methods state their different unit. |
| rounding | Rule used when discarded digits affect the result. |
| initialised | Distinguishes initialised values from the default uninitialised zero. |
| valid | Nullable presence flag, independent of initialisation and numerical zero. |
| exact | No information was lost; conversion APIs may return a separate exactness flag. |

Method names and error identifiers stay in their original form in every language. Locale selection does not change accepted decimal syntax: a comma is not silently converted to a decimal point.
