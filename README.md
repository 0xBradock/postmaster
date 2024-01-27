# Postmaster

> Going through [Zero to Production]()

![build](https://github.com/0xbradock/postmaster/actions/workflows/build.yaml/badge.svg)

# Known Issues

- Missing a `tear_down` method in the e2e tests to `DROP` the created databases ([sqlx::test](https://docs.rs/sqlx/latest/sqlx/attr.test.html)).
