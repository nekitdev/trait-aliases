//! Tests that lifetimes and higher-ranked trait bounds are supported, for instance with `serde`.

use serde::{Deserialize, Serialize};

use trait_aliases::trait_aliases;

trait_aliases! {
    trait SerializableFor<'de> = Serialize + Deserialize<'de>;

    trait Serializable = for<'de> SerializableFor<'de>;
}

fn main() {}
