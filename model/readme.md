# Model documentation
* http://nasdaqtrader.com/content/technicalsupport/specifications/TradingProducts/Ouch5.0.pdf

# Local build
```shell
cargo nextest run --all-features
cargo nextest run --examples
cargo test --doc
cargo doc
cargo clippy --all-features -- --deny warnings
```