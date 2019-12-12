use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct KthLargest {
    k: usize,
    elements: BinaryHeap<Reverse<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut kth_largest = Self {
            k: k as usize,
            elements: BinaryHeap::with_capacity(k as usize),
        };

        for num in nums {
            kth_largest.add(num);
        }

        kth_largest
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.elements.len() < self.k {
            self.elements.push(Reverse(val));
        } else if self.elements.peek() > Some(&Reverse(val)) {
            self.elements.pop();
            self.elements.push(Reverse(val));
        }

        match self.elements.peek() {
            Some(&Reverse(x)) => x,
            None => 0,
        }
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
