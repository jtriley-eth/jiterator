use crate::{
    map::Map,
    filter::Filter,
};

/// Jiterator, like an Iterator but with a J in front.
pub trait Jiterator {
    /// Item over which to iterate.
    type Item;

    /// Optionally get the next item.
    fn next(&mut self) -> Option<Self::Item>;

    /// Find the first item that satisfies the predicate.
    fn find<P>(&mut self, mut predicate: P) -> Option<Self::Item>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        while let Some(x) = self.next() {
            if predicate(&x) {
                return Some(x);
            }
        }
        None
    }

    /// Converts the Jiterator to a Map Jiterator with a function
    fn map<F, B>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        Map::new(self, f)
    }

    /// Converts the Jiterator to a Filter Jiterator with a predicate.
    fn filter<P>(self, predicate: P) -> Filter<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        Filter::new(self, predicate)
    }

    /// Fold the Jiterator into a single value based on an initial value and a function.
    fn fold<B, F>(mut self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        let mut accum = init;
        while let Some(x) = self.next() {
            accum = f(accum, x);
        }
        accum
    }
}
