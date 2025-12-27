//! Tests that `where` clauses work.

use core::fmt::{Debug, Display};

use trait_aliases::trait_aliases;

trait_aliases! {
    trait DD = where Self: Debug + Display;
}

fn test_both<T: DD>() {
    fn test_debug<D: Debug>() {}
    fn test_display<D: Display>() {}

    test_debug::<T>();
    test_display::<T>();
}

fn main() {}
