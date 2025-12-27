//! Tests that the identtifier `__T`, reserved for blanket implementations,
//! can not appear in the macro input.

use trait_aliases::trait_aliases;

trait_aliases! {
    trait __T = Sized;
}

fn main() {}
