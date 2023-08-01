use jiter::Jiterator;

mod filter;
mod jiter;
mod map;

/// Simple Vector Wrapper
pub struct List<T>(Vec<T>);

impl<T> List<T> {
    /// Create a new List.
    pub fn new() -> Self {
        List(Vec::new())
    }

    /// Push an item to the List.
    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }
}

impl<T> Jiterator for List<T>
where
    T: Sized,
{
    /// List's Item is the same as the inner Vec's element type.
    type Item = T;

    /// Get the next item from the List.
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

fn main() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(5);
    list.push(6);

    let list_jiterator = list
        .filter(|x| x % 2 == 0)
        .map(|x| x * 4)
        .map(|x| x - 1);

    print_type_of(&list_jiterator);

    let sum = list_jiterator.fold(0, |acc, x| acc + x);

    println!("Sum: {sum}");
}

fn print_type_of<T>(_: &T) {
    println!("Type: {}", std::any::type_name::<T>())
}
