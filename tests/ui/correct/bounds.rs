//! Tests that bounds can be applied to generic parameters and associated types.

use trait_aliases::trait_aliases;

trait_aliases! {
    trait GenericIteratorItemSend<T: Send> = Iterator<Item = T>;

    trait IteratorItemSend = Iterator<Item: Send>;
}

fn test_send<T: Send, G: GenericIteratorItemSend<T>>() {
    fn check<I: IteratorItemSend>() {}

    check::<G>();
}

fn main() {}
