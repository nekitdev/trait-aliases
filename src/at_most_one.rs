pub enum OneOrTwo<T> {
    One(T),
    Two(T, T),
}

pub struct ExactlyOneError<I: Iterator> {
    start: Option<OneOrTwo<I::Item>>,
    iterator: I,
}

impl<I: Iterator> ExactlyOneError<I> {
    const fn new(start: Option<OneOrTwo<I::Item>>, iterator: I) -> Self {
        Self { start, iterator }
    }

    pub const fn two(one: I::Item, two: I::Item, iterator: I) -> Self {
        Self::new(Some(OneOrTwo::Two(one, two)), iterator)
    }
}

impl<I: Iterator> Iterator for ExactlyOneError<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.start.take() {
            Some(OneOrTwo::Two(next, after)) => {
                self.start = Some(OneOrTwo::One(after));

                Some(next)
            }
            Some(OneOrTwo::One(next)) => Some(next),
            None => self.iterator.next(),
        }
    }
}

pub trait AtMostOne: Iterator + Sized {
    fn at_most_one(mut self) -> Result<Option<Self::Item>, ExactlyOneError<Self>> {
        self.next().map_or(Ok(None), |one| match self.next() {
            Some(two) => Err(ExactlyOneError::two(one, two, self)),
            None => Ok(Some(one)),
        })
    }
}

impl<I: Iterator> AtMostOne for I {}
