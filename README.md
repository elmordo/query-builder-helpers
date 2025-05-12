# Query Builder Helpers

The library provides some useful shorthands for working with
the [QueryBuilder](https://docs.rs/sqlx/latest/sqlx/struct.QueryBuilder.html) of the [sqlx](https://docs.rs/sqlx/latest/sqlx/) library.

**WARNING!: This library is under development!**

## Main features of the library

* pagination using the `Pagination` trait and `PaginationSettings` struct,
* where collection helpers with the `Condtions` trait.

## Where condition helper

The `Condtions` trait contains helpers for using following where conditions:

* `(NOT) IN`
* `LIKE`
