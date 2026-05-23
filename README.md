# pizza-analysis-french

French language analysis with elision handling, light/minimal stemmers, accent normalization, and stop words.

Part of the [Pizza](https://pizza.rs) search engine.

## Components

| Name | Type | Description |
|------|------|-------------|
| `french_elision` | Token Filter | Removes French elisions (l', d', qu', etc.) |
| `french_stem` | Token Filter | French light stemmer with accent normalization (à/â→a, ç→c, è/é/ê/ë→e, etc.) |
| `french_minimal_stem` | Token Filter | French minimal stemmer — less aggressive suffix removal |
| `french_stop` | Token Filter | French stop words filter (154 words) |
| `french` | Analyzer | Full pipeline: lowercase → elision → stop → stem |

## Usage

### Built-in Analyzer

```json
{
  "analyzer": {
    "type": "french"
  }
}
```

### Custom Pipeline

```json
{
  "analyzer": {
    "type": "custom",
    "tokenizer": "standard",
    "filter": ["french_elision", "french_stem", "french_minimal_stem", "french_stop"]
  }
}
```

## License

MIT — see [LICENSE](LICENSE).

## Related Crates

- [analysis-core](https://github.com/pizza-rs/analysis-core) — Core analysis components and pipeline
- [analysis-icu](https://github.com/pizza-rs/analysis-icu) — ICU Unicode normalization and tokenization
- [analysis-english](https://github.com/pizza-rs/analysis-english) — English analysis
- [analysis-all](https://github.com/pizza-rs/analysis-all) — Meta-crate registering all analyzers
