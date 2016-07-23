# wkhtmltox-sys

Auto-generated bindings to libwkhtmltox

See [wkhtmltopdf-rs](https://github.com/anowell/wkhtmltopdf-rs) for the abstracted higher-level bindings.

You'll need libwkhtmltox installed.

[Documentation](https://anowell.github.io/wkhtmltox-sys/wkhtmltox_sys/)

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
