# Changelog

<!-- changelogging: start -->

## [0.2.0](https://github.com/nekitdev/trait-aliases/tree/v0.2.0) (2025-12-27)

### Changes

- The input of `trait_aliases!` is now traversed fully, and any occurrences of the reserved `__T`
  identifier cause compilation to fail; that is, snippets like

  ```rust
  use trait_aliases::trait_aliases;

  trait_aliases! {
      trait __T = Sized;
  }
  ```

  now fail with

  ```text
  identifier `__T` is reserved for blanket implementations
  ```

### Internal

- All the internal functionality has been documented. It can be viewed locally via

  ```console
  $ cargo doc --document-private-items
  ```

- UI [tests](https://github.com/nekitdev/trait-aliases/tree/main/tests) have been added
  in order to ensure correctness.

## [0.1.0](https://github.com/nekitdev/trait-aliases/tree/v0.1.0) (2025-12-24)

Initial release.
