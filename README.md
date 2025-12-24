# `trait-aliases`

[![License][License Badge]][License]
[![Version][Version Badge]][Crate]
[![Downloads][Downloads Badge]][Crate]
[![Documentation][Documentation Badge]][Documentation]
[![Test][Test Badge]][Actions]

> *Trait aliases.*

## Installation

### `cargo`

You can add `trait-aliases` as a dependency with the following command:

```console
$ cargo add trait-aliases
```

Or by directly specifying it in the configuration like so:

```toml
[dependencies]
trait-aliases = "0.1.0"
```

Alternatively, you can add it directly from the source:

```toml
[dependencies.trait-aliases]
git = "https://github.com/nekitdev/trait-aliases.git"
```

## Example

> Ever felt tired of writing `T: Send + Sync + 'static` over and over when working with `async`
> in multi-threaded scenarios?

Simply define an alias without blanket implementation boilerplate!

```rust
use trait_aliases::trait_aliases;

trait_aliases! {
    /// Working in multi-threaded `async` contexts often requires these.
    pub trait SSS = Send + Sync + 'static;
}
```

This crate will generate the `SSS` trait with the provided bounds, and implement it for any type
satisfying them:

```rust
/// Working in multi-threaded `async` contexts often requires these.
pub trait SSS: Send + Sync + 'static {}

/// Blanket implementation of [`SSS`] for all types satisfying its bounds.
impl<__T> SSS for __T where __T: Send + Sync + 'static + ?Sized {}
```

## Note

Please *never* use `__T` in your generic parameters, as it is reserved for the
blanket implementation.

Failing to do so will result in collisions at best, and hard-to-debug errors,
migraines or even spontaneous combustion at worst.

## Documentation

You can find the documentation [here][Documentation].

## Support

If you need support with the library, you can send an [email][Email].

## Changelog

You can find the changelog [here][Changelog].

## Security Policy

You can find the Security Policy of `trait-aliases` [here][Security].

## Contributing

If you are interested in contributing to `trait-aliases`, make sure to take a look at the
[Contributing Guide][Contributing Guide], as well as the [Code of Conduct][Code of Conduct].

## License

`trait-aliases` is licensed under the MIT License terms. See [License][License] for details.

[Email]: mailto:support@nekit.dev

[Discord]: https://nekit.dev/chat

[Actions]: https://github.com/nekitdev/trait-aliases/actions

[Changelog]: https://github.com/nekitdev/trait-aliases/blob/main/CHANGELOG.md
[Code of Conduct]: https://github.com/nekitdev/trait-aliases/blob/main/CODE_OF_CONDUCT.md
[Contributing Guide]: https://github.com/nekitdev/trait-aliases/blob/main/CONTRIBUTING.md
[Security]: https://github.com/nekitdev/trait-aliases/blob/main/SECURITY.md

[License]: https://github.com/nekitdev/trait-aliases/blob/main/LICENSE

[Crate]: https://crates.io/crates/trait-aliases
[Documentation]: https://docs.rs/trait-aliases

[License Badge]: https://img.shields.io/crates/l/trait-aliases
[Version Badge]: https://img.shields.io/crates/v/trait-aliases
[Downloads Badge]: https://img.shields.io/crates/dr/trait-aliases
[Documentation Badge]: https://img.shields.io/docsrs/trait-aliases
[Test Badge]: https://github.com/nekitdev/trait-aliases/workflows/test/badge.svg
