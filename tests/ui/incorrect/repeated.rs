use trait_aliases::trait_aliases;

trait_aliases! {
    #[trait_alias(T, U, V)]
    trait S = Sized;
}

fn main() {}
