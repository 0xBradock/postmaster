# Postmaster

> Going through [Zero to Production]()

<div align="center">
  [![test-badge]](https://github.com/0xbradock/postmaster/actions/workflows/general.yaml/badge.svg)
</div>

# Known Issues

- Missing a `tear_down` method in the e2e tests to `DROP` the created databases ([sqlx::test](https://docs.rs/sqlx/latest/sqlx/attr.test.html)).