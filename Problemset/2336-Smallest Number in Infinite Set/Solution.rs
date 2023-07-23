use std::collections::BinaryHeap;
use std::collections::HashSet;

struct SmallestInfiniteSet {
    min_infinite: i32,
    heap: BinaryHeap<i32>,
    set: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {
    fn new() -> Self {
        Self {
            min_infinite: 1,
            heap: BinaryHeap::new(),
            set: HashSet::new(),
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        match self.heap.pop() {
            Some(x) => self.set.take(&(-x)).unwrap(),
            None => {
                self.min_infinite += 1;
                self.min_infinite - 1
            }
        }
    }

    fn add_back(&mut self, num: i32) {
        if num < self.min_infinite && self.set.insert(num) {
            self.heap.push(-num);
        }
    }
}

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */
