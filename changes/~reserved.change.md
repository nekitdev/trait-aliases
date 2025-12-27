The input of `trait_aliases!` is now traversed fully, and any occurrences of the reserved `__T`
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
