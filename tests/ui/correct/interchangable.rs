//! Tests that aliases with the same bounds are interchangeable.

use trait_aliases::trait_aliases;

trait_aliases! {
    trait ThreadSafeA = Send + Sync;
    trait ThreadSafeB = Send + Sync;
}

fn test_a_to_b<A: ThreadSafeA>() {
    fn check<B: ThreadSafeB>() {}

    check::<A>();
}

fn test_b_to_a<B: ThreadSafeB>() {
    fn check<A: ThreadSafeA>() {}

    check::<B>();
}

fn main() {}
