use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct LUPrefix {
    heap: BinaryHeap<Reverse<i32>>,
    longest_uploaded_prefix: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LUPrefix {
    fn new(n: i32) -> Self {
        Self {
            heap: BinaryHeap::new(),
            longest_uploaded_prefix: 0,
        }
    }

    fn upload(&mut self, video: i32) {
        self.heap.push(Reverse(video));
    }

    fn longest(&mut self) -> i32 {
        while self.heap.peek().unwrap_or(&Reverse(0)).0 == self.longest_uploaded_prefix + 1 {
            self.heap.pop();
            self.longest_uploaded_prefix += 1;
        }

        self.longest_uploaded_prefix
    }
}

/**
 * Your LUPrefix object will be instantiated and called as such:
 * let obj = LUPrefix::new(n);
 * obj.upload(video);
 * let ret_2: i32 = obj.longest();
 */
