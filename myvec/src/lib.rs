use std::alloc;
use std::ptr::NonNull;

pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        return Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        };
    }

    pub fn push(&mut self, item: T) {
        assert_ne!(std::mem::size_of::<T>(), 0, "No zero sized types");

        if self.capacity == 0 {
            let layout = alloc::Layout::array::<T>(4).expect("Could not reserve the space!");
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
            let ptr = NonNull::new(ptr).expect("ptr could be null");
            unsafe { ptr.as_ptr().write(item) };
            self.ptr = ptr;
            self.capacity = 4;
            self.len = 1;
        }
    }

    pub fn capacity(&self) -> usize {
        return self.capacity;
    }

    pub fn len(&self) -> usize {
        return self.len;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let mut vec = Vec::new();
        // vec.push(2);
        // vec.push(3);
        // vec.push(4);
        // vec.push(5);

        let mut vec: MyVec<usize> = MyVec::new();
        vec.push(1);

        assert_eq!(vec.capacity(), 0);
        assert_eq!(vec.len(), 0);
    }
}
