use alloc::vec::Vec;
use core::fmt::Debug;

pub trait Allocator<T> {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<T>;
    fn dealloc(&mut self, item: T);
}

pub struct RecycleAllocator<I: Iterator> {
    current: Option<I>,
    recycled: Vec<I::Item>,
}

impl<I: Iterator> RecycleAllocator<I> {
    pub fn init(&mut self, begin: I) {
        self.current = Some(begin);
    }
}
impl<I: Iterator<Item = T>, T: PartialEq + Debug> Allocator<T> for RecycleAllocator<I> {
    fn new() -> Self {
        Self {
            current: None,
            recycled: Vec::new(),
        }
    }
    fn alloc(&mut self) -> Option<T> {
        if let Some(p) = self.recycled.pop() {
            Some(p)
        } else if let Some(i) = self.current {
            i.next()
        } else {
            None
        }
    }
    fn dealloc(&mut self, item: T) {
        // validity check
        assert!(
            !self.recycled.iter().any(|v| *v == item),
            "Item {:?} has not been allocated!",
            item
        );
        // recycle
        self.recycled.push(item);
    }
}
