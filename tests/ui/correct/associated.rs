//! Tests that associated types are supported.

use trait_aliases::trait_aliases;

trait_aliases! {
    trait Bytes = Iterator<Item = u8>;
}

fn test_bytes<B: Bytes>(_bytes: B) {}

fn main() {
    test_bytes("Hello, world!".bytes());

    let farewell = b"Bye, world!";

    test_bytes(farewell.iter().copied());
}
