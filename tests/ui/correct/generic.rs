//! Tests that const and type generics as well as concrete types compile.

use trait_aliases::trait_aliases;

const COUNT: usize = 4;

trait_aliases! {
    trait IntoArray<T, const N: usize> = Into<[T; N]>;

    trait IntoBytes<const N: usize> = IntoArray<u8, N>;

    trait IntoCount = IntoBytes<COUNT>;
}

fn test_into_count<T: IntoCount>(_value: T) {}

fn main() {
    let bytes: [u8; COUNT] = [13, 34, 42, 69];

    test_into_count(bytes);
}
