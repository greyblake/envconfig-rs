#### Unreleased
* [breaking] Nested config fields now needs to be marked with #[envconfig(nested = true)]
* Environment variable can be automatically derived from a field name (e.g. `db_host` will be tried to loaded from `DB_HOST` env var)
* Add install step in README

#### v0.10.0 - 2021-06-8
* Add `init_from_hashmap` to initialize config from `HashMap<String, String>` in unit tests

#### v0.9.1 - 2019-10-09
* Get rid of thiserror dependency

#### v0.9.0 - 2019-10-05
* Use rust edition 2018
* Make envconfig re-export Envconfig macro from envconfig_derive (no need to use envconfig_derive explicitly anymore)
* Deprecate `init()` function in favor of `init_from_env()`

#### v0.8.0 - 2019-03-31
* Update `syn`, `quote` and `proc-macro2` dependencies to be `1.x.x`

#### v0.7.0 - 2019-02-20
* Use `thiserror` for errors

#### v0.6.0 - 2019-12-22
* Support nested structures

#### v0.5.1 - 2019-04-14
* Fix `Result` overlapping bug

#### v0.5.0 - 2018-09-25
* Support `default` attribute to specify default values

#### v0.4.0 - 2018-09-22
* Support of `Option<T>` types
* Rewrite `envconfig_derive` to use the latests versions of `syn` and `quote` crates
* Improve error messages on panics
* Add `skeptic` to generate tests based on README code examples

#### v0.3.0 - 2018-09-16
* [breaking] Use `envconfig` attribute instead of `from` in the derive macro
* [breaking] Remove init_or_die() function from Envconfig trait
* [breaking] In envconfig_derive: rename function envconfig() -> derive()
* [improvement] Add better documentation to the crate

#### v0.2.0 - 2018-09-13
* [breaking] Use derive macro instead of macro_rules

#### v0.1.0 - 2018-08-18
* First public release
