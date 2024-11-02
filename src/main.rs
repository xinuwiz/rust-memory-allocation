pub struct Block<T> {
    values: [Option<T>; 8],
}

impl<T> Default for Block<T> {
    fn default() -> Self {
        Block {
            values: Default::default(),
        }
    }
}

impl<T> Drop for Block<T> {
    fn drop(&mut self) {
        self.values = Default::default();
    }
}

impl<T> Block<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn allocate<F>(&mut self, closure: F) -> &T
    where
        F: FnOnce() -> T,
    {
        for slot in self.values.iter_mut() {
            if slot.is_none() {
                *slot = Some(closure());
                return slot.as_ref().unwrap()
            }
        }

        panic!("out of memory");
    }
}

fn main() { }
