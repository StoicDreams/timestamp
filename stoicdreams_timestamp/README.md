# [stoicdreams_timestamp](docsrs): TimeStamp, StopWatch, DateTime, and Time

[![Timestamp GitHub Actions][gh-image]][gh-checks]
[![Timestamp on crates.io][cratesio-image]][cratesio]
[![Timestamp on docs.rs][docsrs-image]][docsrs]

[gh-image]: https://github.com/stoicdreams/timestamp/actions/workflows/deploy.yml/badge.svg
[gh-checks]: https://github.com/stoicdreams/timestamp/actions?query=branch%3Amain
[cratesio-image]: https://img.shields.io/crates/v/stoicdreams_timestamp.svg
[cratesio]: https://crates.io/crates/stoicdreams_timestamp
[docsrs-image]: https://docs.rs/stoicdreams_timestamp/badge.svg
[docsrs]: https://docs.rs/stoicdreams_timestamp

## About

This project was created after discovering a security vulnerability exists (found using `cargo audit`) in the commonly used [chrono](https://crates.io/crates/chrono) cargo crate as of the then current version `0.4.24`. The only actual functionality we wanted at the time was to be able to get a time stamp of the current date and time in UTC for the purposes of saving to a [SurrealDB](https://surrealdb.com/) database, and the ability to format our time stamps when displaying in various UI applications.

## Goals / Features

- Store times down to the millisecond.
- Store data as 64 bit integer to ensure date storage within the range of humanities lifetime.
- Store data as signed integer to allow potential for supporting BC dates and times.
- Store DateTime values with 0 representing midnight 0 AD instead of the more commonly used Unix Epoch at midnight 1970 AD.
- Default string rendering for date and time is in UTC and follows the ISO 8601 standard.
- Added StopWatch and PreciseTime for tracking precise timings - typically for measuring and tracking performance.

## Future Goals / Features

- Implement support for BC dates and times.
- Implement support for 128bit storage - using feature flag to enable.

## Getting Started

Update your `Cargo.toml` file to add the `stoicdreams_timestamp` crate as a dependency.

```toml
[package]
name = "name_of_your_app"
version = "0.1.0"
edition = "2021"

[dependencies]
stoicdreams_timestamp = "0.1.2"
```

Use prelude for simple access to all structs and methods.

```rust
use stoicdreams_timestamp::prelude::*;
```

## Other Recommended Tools

Crate | Install Command | Example Run Command
--- | --- | ---
rustfmt | rustup component add rustfmt | cargo fmt
clippy | rustup component add clippy | cargo clippy
cargo-audit | cargo install cargo-audit | cargo audit

## Other Resources

[Rust Docs](https://www.rust-lang.org/)
[Yew Docs](https://yew.rs/)

## Author

**[Erik Gassler](https://www.erikgassler.com) - [Stoic Dreams](https://www.stoicdreams.com)** - Forging solutions for tomorrow's software development.

**Support** - Visit [Stoic Dreams' GitHub Sponsor page](https://github.com/sponsors/StoicDreams) if you would like to provide support to Stoic Dreams.

## License

[MIT](../LICENSE)
