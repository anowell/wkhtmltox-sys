# wkhtmltox-sys

Auto-generated bindings to libwkhtmltox.

| Resource            | Link                                                                                                                     |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| Crate               | [![Crates.io](https://img.shields.io/crates/v/wkhtmltox-sys.svg?maxAge=2592000)](https://crates.io/crates/wkhtmltox-sys) |
| Documentation       | [Cargo docs](https://anowell.github.io/wkhtmltox-sys/wkhtmltox_sys/)                                                     |
| High-level bindings | [wkhtmltopdf-rs](https://github.com/anowell/wkhtmltopdf-rs)                                                              |
| Upstream            | [wkhtmltopdf.org](http://wkhtmltopdf.org/)                                                                               |

## Generating

This is generated with bindgen:

```sh
bindgen --dynamic-loading wkhtmltox include/pdf.h > src/pdf.rs -- -DBUILDING_WKHTMLTOX=1
bindgen --dynamic-loading wkhtmltox include/image.h > src/image.rs -- -DBUILDING_WKHTMLTOX=1
cargo fmt
```

## Examples

The Rust example is built with cargo:

```sh
cargo build --example convert_string
./target/debug/examples/convert_string
```

```sh
cargo build --example convert_url
./target/debug/examples/convert_url
```
