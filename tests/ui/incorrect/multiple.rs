//! Tests that *all* occurrences of `__T` passed to `trait_aliases!`
//! are reported as errors.

use trait_aliases::trait_aliases;

trait_aliases! {
    trait Convertible<__T> = From<__T> + Into<__T>;
}

fn main() {}
