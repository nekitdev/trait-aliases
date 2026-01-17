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
trait-aliases = "0.3.0"
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

## Attribute

The expansion can be customized via applying the `#[trait_alias]` attribute:

```rust
use trait_aliases::trait_aliases;

trait_aliases! {
    /// Working in multi-threaded `async` contexts often requires these.
    #[trait_alias(
        T,
        doc = "Implemented for any type that is [`Send`], [`Sync`] and `'static`",
        doc = "(meaning it does not contain non-static lifetimes)."
    )]
    pub trait SSS = Send + Sync + 'static;
}
```

Expands to the following:

```rust
/// Working in multi-threaded `async` contexts often requires these.
pub trait SSS: Send + Sync + 'static {}

/// Implemented for any type that is [`Send`], [`Sync`] and `'static`
/// (meaning it does not contain non-static lifetimes).
impl<T> SSS for T where T: Send + Sync + 'static + ?Sized {}
```

## Note

The blanket identifier is essential to correct code generation, therefore *any* occurrences
of the selected identifier will result in compilation errors.

When the identifier is supplied to `#[trait_alias]`, for instance:

```rust
use trait_aliases::trait_aliases;

trait_aliases! {
    #[trait_alias(T)]
    trait Convertible<T> = From<T> + Into<T>;
}
```

will cause compilation to fail with several errors like:

```text
identifier `T` is reserved for blanket implementations
```

pointing to every occurrence of `T` within the trait alias definition.

Otherwise, the default `__T` is used, therefore examples like:

```rust
use trait_aliases::trait_aliases;

trait_aliases! {
    trait __T = Sized;
}
```

fail with the following error:

```text
error: identifier `__T` is reserved for blanket implementations
 --> src/main.rs
  |
  |     trait __T = Sized;
  |           ^^^
```

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
