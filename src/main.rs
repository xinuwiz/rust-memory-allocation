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

    pub fn allocate<F>(&mut self, closure: F) -> Result<&T, &str>
    where
        F: FnOnce() -> T,
    {
        for slot in self.values.iter_mut() {
            if slot.is_none() {
                *slot = Some(closure());
                return Ok(slot.as_ref().unwrap());
            }
        }

        Err("out of memory")
    }

    pub fn free(&mut self, slot: usize) -> Result<(), &str> {
        if slot >= self.values.len() {
            Err("invalid slot")
        } else {
            self.values[slot] = None;
            Ok(())
        }
    }
}

fn main() {
}
