# wkhtmltox-sys

Auto-generated bindings to libwkhtmltox. Depends on libwkhtmltox being installed.

Resource  | Link    
----- | -----
Crate | [![Crates.io](https://img.shields.io/crates/v/wkhtmltox-sys.svg?maxAge=2592000)](https://crates.io/crates/wkhtmltox-sys)
Documentation | [Cargo docs](https://anowell.github.io/wkhtmltox-sys/wkhtmltox_sys/)
High-level bindings | [wkhtmltopdf-rs](https://github.com/anowell/wkhtmltopdf-rs)
Upstream | [wkhtmltopdf.org](http://wkhtmltopdf.org/)


## Generating

This is generated with bindgen:

```
$ bindgen --link=dynamic=wkhtmltox include/pdf.h > src/pdf.rs
$ bindgen --link=dynamic=wkhtmltox include/image.h > src/image.rs
$ cargo fmt
```

## Examples

The Rust example is built with cargo:

```
cargo test
target/debug/examples/convert-string
```

The C example can be built with gcc:

```
gcc -g -Wall examples/convert-string.c -o target/convert-string-c -lwkhtmltox
target/convert-string-c
```
