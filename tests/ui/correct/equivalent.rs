//! Tests that the generated trait alias and its bounds are equivalent.

use trait_aliases::trait_aliases;

trait_aliases! {
    trait SSS = Send + Sync + 'static;
}

fn alias_to_bound<S: SSS>() {
    fn check<T: Send + Sync + 'static>() {}

    check::<S>();
}

fn bound_to_alias<T: Send + Sync + 'static>() {
    fn check<S: SSS>() {}

    check::<T>();
}

fn main() {}
