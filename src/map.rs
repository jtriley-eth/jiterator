use crate::jiter::Jiterator;

/// Map Jiterator
pub struct Map<J, F> {
    /// Jiterator over which to map.
    jiter: J,
    /// Function with which to map.
    f: F,
}

impl<J, F> Map<J, F> {
    /// Create a new Map Jiterator.
    pub fn new(jiter: J, f: F) -> Self {
        Map { jiter, f }
    }
}

impl<B, J: Jiterator, F> Jiterator for Map<J, F>
where
    F: FnMut(J::Item) -> B,
{
    /// Map uses the same Item as the inner Jiterator
    type Item = B;

    /// Get the next item from the inner Jiterator and map the function over it.
    fn next(&mut self) -> Option<Self::Item> {
        self.jiter.next().map(&mut self.f)
    }
}
