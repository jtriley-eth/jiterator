use crate::jiter::Jiterator;

/// Filter Jiterator
pub struct Filter<J, P> {
    /// Jiterator over which to filter.
    jiter: J,
    /// Predicate with which to filter.
    predicate: P,
}

impl<J, P> Filter<J, P> {
    /// Create a new Filter Jiterator.
    pub fn new(jiter: J, predicate: P) -> Self {
        Self { jiter, predicate }
    }
}

impl<J: Jiterator, P> Jiterator for Filter<J, P>
where
    P: FnMut(&J::Item) -> bool,
{
    /// Filter uses the same Item as the inner Jiterator
    type Item = J::Item;

    /// Get the next item that satisfies the predicate from the inner Jiterator.
    fn next(&mut self) -> Option<J::Item> {
        self.jiter.find(&mut self.predicate)
    }
}
