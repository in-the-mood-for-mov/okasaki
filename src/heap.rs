pub trait Heap<'a, T> {
    fn place(&'a self, T) -> &'a T;
}

#[cfg(test)]
pub mod vec_heap {
    use super::Heap;
    use std::cell::UnsafeCell;

    pub struct VecHeap<T>(UnsafeCell<Vec<T>>);

    impl<T> VecHeap<T> {
        pub fn new() -> VecHeap<T> {
            VecHeap(UnsafeCell::new(Vec::new()))
        }
    }

    impl<'a, T> Heap<'a, T> for VecHeap<T> {
        fn place(&'a self, t: T) -> &'a T {
            // hack
            unsafe {
                let inner = &mut *self.0.get();
                inner.push(t);
                inner.last().unwrap()
            }
        }
    }
}
