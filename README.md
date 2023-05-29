# [stoicdreams_timestamp](docsrs): Developer Docs

[![Timestamp GitHub Actions][gh-image]][gh-checks]
[![Timestamp on crates.io][cratesio-image]][cratesio]
[![Timestamp on docs.rs][docsrs-image]][docsrs]

[gh-image]: https://github.com/stoicdreams/timestamp/actions/workflows/deploy.yml/badge.svg
[gh-checks]: https://github.com/stoicdreams/timestamp/actions?query=branch%3Amain
[cratesio-image]: https://img.shields.io/crates/v/stoicdreams_timestamp.svg
[cratesio]: https://crates.io/crates/stoicdreams_timestamp
[docsrs-image]: https://docs.rs/stoicdreams_timestamp/badge.svg
[docsrs]: https://docs.rs/stoicdreams_timestamp

## Projects

### [stoicdreams_timestamp](./stoicdreams_timestamp/README.md)

A lightweight library that includes `TimeStamp`, `DateTime`, and `Time` structs intended for simple date/time time stamping for database records - explicitely to use in `SurrealDB` databases.

## Getting Started

### Install Required Dev Dependencies

Follow [instructions here to install Rust](https://rustup.rs/) for your system.

Make sure rust is up to date

```powershell
rustup update
```

Install Rusts automatic formatting tool.

```powershell
rustup component add rustfmt
```

Then you can run this command to apply formatting to your project.

```powershell
cargo fmt
```

Install Rusts Clippy linting tool.

```powershell
rustup component add clippy
```

Then you can run this command to find additional linting errors.

```powershell
cargo clippy
```

Install Cargo Audit

```powershell
cargo install cargo-audit
```

Then you can run this command to audit your rust projects.

```powershell
cargo audit
```

### Increment Versions after updates

If a major or minor release update is required, then first manually apply the version update to `webui/Cargo.toml`, making sure to reset lower versions to 0.

Run this script to increment the patch version and apply the new version to any references / docs.

```powershell
# From the workspace root folder
.\IncrementVersion.ps1
```

### Publish Latest Updates to [crates.io](https://crates.io/crates/stoicdreams_timestamp/)

First, commit any changes to Git.

Then, run the publish command from the `stoicdreams_timestamp` folder to publish.

```powershell
# From the webui folder
cargo publish
```

## Other Resources

[Rust Docs](https://www.rust-lang.org/)
[Yew Docs](https://yew.rs/)

## Author

**[Erik Gassler](https://www.erikgassler.com) - [Stoic Dreams](https://www.stoicdreams.com)** - Just a simpleton who likes making stuff with bits and bytes.

**Support** - Visit [Stoic Dreams' GitHub Sponsor page](https://github.com/sponsors/StoicDreams) if you would like to provide support to Stoic Dreams.

## License

[MIT](./LICENSE)
