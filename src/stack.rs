use heap::{Heap};

pub trait Stack<'a>: Sized {
    type E;

    fn empty(heap: &'a Heap<'a, Self>) -> &'a Self;
    fn empty_p(&'a self) -> bool;

    fn cons(&'a self, heap: &'a Heap<'a, Self>, e: Self::E) -> &'a Self;
    fn head(&'a self) -> Option<&'a Self::E>;
    fn tail(&'a self) -> Option<&'a Self>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum PersistentList<'a, E: 'a> {
    Nil,
    Cons(E, &'a PersistentList<'a, E>),
}

impl<'a, E> Stack<'a> for PersistentList<'a, E> {
    type E = E;

    fn empty(heap: &'a Heap<'a, Self>) -> &'a Self {
        heap.place(PersistentList::Nil)
    }

    fn empty_p(&'a self) -> bool {
        match self {
            &PersistentList::Nil => true,
            &PersistentList::Cons(..) => false,
        }
    }

    fn cons(&'a self, heap: &'a Heap<'a, Self>, e: Self::E) -> &'a Self {
        heap.place(PersistentList::Cons(e, self))
    }

    fn head(&'a self) -> Option<&'a Self::E> {
        match self {
            &PersistentList::Nil => None,
            &PersistentList::Cons(ref head, _) => Some(head),
        }
    }

    fn tail(&'a self) -> Option<&'a Self> {
        match self {
            &PersistentList::Nil => None,
            &PersistentList::Cons(_, ref tail) => Some(tail),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use heap::vec_heap::VecHeap;

    #[test]
    fn persistent_list() {
        let heap = VecHeap::new();
        let list0: &PersistentList<_> = Stack::empty(&heap);
        let list1 = list0.cons(&heap, 3);
        let list2 = list1.cons(&heap, 2);
        let list3 = list2.cons(&heap, 1);
        assert_eq!(list3.head(), Some(&1));
        assert_eq!(list3.tail(), Some(list2));
    }
}
