//! Tests that aliases can be combined into other aliases.

use trait_aliases::trait_aliases;

trait_aliases! {
    trait ThreadSafe = Send + Sync;

    trait Bytes = Iterator<Item = u8>;

    trait ThreadSafeBytes = ThreadSafe + Bytes;
}

fn test_combination<B: ThreadSafeBytes>(_bytes: B) {}

fn main() {
    let bytes: [u8; _] = [13, 42, 69];

    test_combination(bytes.into_iter());
}
