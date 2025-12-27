//! Tests that the `?Sized` bound is added in the blanket implementation.
//!
//! In case said bound is not added, the generated implementation of `Alias` would look like
//!
//! ```
//! impl<__T> Alias for __T where __T: Test {}
//! ```
//!
//! and, while `Test` is implemented for `[T]`, `Alias` would be not, causing compilation error.

use trait_aliases::trait_aliases;

trait Test {
    fn test(&mut self);
}

impl<T> Test for [T] {
    fn test(&mut self) {}
}

trait_aliases! {
    trait Alias = Test;
}

fn test_alias<T>() {
    fn check<A: Alias + ?Sized>() {}

    check::<[T]>();
}

fn main() {}
