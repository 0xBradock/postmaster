# Postmaster

> Going through [Zero to Production]()

![build](https://github.com/0xbradock/postmaster/actions/workflows/build.yaml/badge.svg)

# Tooling

HTTP:

- [actix](https://actix.rs/): Web framework
- [reqwest](https://docs.rs/reqwest/latest/reqwest/): High-level HTTP client

Log:

- [tracing](https://docs.rs/tracing/latest/tracing/): Scoped, structural logging and diagnostics system
- [tracing-actix-web](https://docs.rs/tracing-actix-web/latest/tracing_actix_web/): Telemetry middleware for actix-web applications
- [secrecy](https://docs.rs/secrecy/0.8.0/secrecy/): Wrapper to handle secret values
- [sqlx](https://docs.rs/sqlx/latest/sqlx/): Async SQL toolkit

Security:

- [rustsec.org](https://rustsec.org/): Vulnerability database for the Rust ecosystem

Tests:

- [fake](https://docs.rs/fake/latest/fake/): Generate fake data
- [quickcheck](https://docs.rs/quickcheck/latest/quickcheck/): Random testing for property testing
- [quickcheck_macros](https://crates.io/crates/quickcheck_macros): Macros for `quickcheck`
- [rand](https://docs.rs/rand/latest/rand/): Random number generator
- [validator](https://crates.io/crates/validator): Simple struct validation
- [wiremock](https://github.com/lukemathwalker/wiremock-rs): Mocking HTTP

# Known Issues

- Missing a `tear_down` method in the e2e tests to `DROP` the created databases ([sqlx::test](https://docs.rs/sqlx/latest/sqlx/attr.test.html)).
- Build with smaller Dockerfile:
  - Using with `alpine` and [`rust-musl-builder`](https://github.com/emk/rust-musl-builder)
  - [Repo](https://github.com/johnthagen/min-sized-rust#strip-symbols-from-binary) with tips on how to build smaller image
