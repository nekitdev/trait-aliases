//! Tests that *all* occurrences of `T` passed to `trait_aliases!`
//! are reported as errors.

use trait_aliases::trait_aliases;

trait_aliases! {
    #[trait_alias(T)]
    trait Convertible<T> = From<T> + Into<T>;
}

fn main() {}
