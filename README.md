# archiver-rs

[![Crates.io](https://img.shields.io/crates/v/archiver-rs)](http://crates.io/crates/archiver-rs)
[![Docs.rs](https://docs.rs/archiver-rs/badge.svg)](https://docs.rs/archiver-rs)
[![Crates.io](https://img.shields.io/crates/d/archiver-rs)](http://crates.io/crates/archiver-rs)
[![Crates.io](https://img.shields.io/crates/l/archiver-rs)](https://github.com/JoyMoe/archiver-rs/blob/master/LICENSE)

A library for easy interaction with multiple archive formats

## Usage

```rust
archiver_rs::bzip2::decode("foo.tar.bz2", "foo.tar");
archiver_rs::gzip::decode("foo.tar.gz", "foo.tar");

archiver_rs::tar::contains("foo.tar", "bar.txt");
archiver_rs::tar::extract("foo.tar", "./foo/");
archiver_rs::tar::extract_single("foo.tar", "./foo/", "bar.txt");

archiver_rs::zip::contains("foo.zip", "bar.txt");
archiver_rs::zip::extract("foo.zip", "./foo/");
archiver_rs::zip::extract_single("foo.zip", "./foo/", "bar.txt");
```

## License

The MIT License

More info see [LICENSE](LICENSE)
