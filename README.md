<div align="center">

# 🇫🇷 pizza-analysis-french

**French text analysis plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--french-blue)](https://github.com/pizza-rs/analysis-french)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

French language analysis with elision handling, light stemming, and stop words.
Correctly processes French articles that contract before vowels (l', d', qu').

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `french_elision` | Strip French elided articles (l', d', qu', n', etc.) |
| TokenFilter | `french_light_stem` | French light stemmer (suffix removal) |
| TokenFilter | `french_stop` | French stop words (154 entries) |
| Analyzer | `french` | Full pipeline: lowercase → elision → light_stem → stop |

### Elision Handling

French contracts articles before vowels. The elision filter strips these prefixes:

| Input | After Elision |
|:------|:--------------|
| l'homme | homme |
| d'accord | accord |
| qu'il | il |
| n'est | est |

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_french::register_all(&mut factory);

let analyzer = factory.get_analyzer("french").unwrap();
// "l'étudiant" → ["etudiant"] (after elision + lowercase)
```

## Installation

```toml
[dependencies]
pizza-analysis-french = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["french"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
