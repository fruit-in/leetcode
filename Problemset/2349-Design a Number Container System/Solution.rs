use std::collections::BinaryHeap;
use std::collections::HashMap;

struct NumberContainers {
    nums: HashMap<i32, i32>,
    indices: HashMap<i32, BinaryHeap<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {
    fn new() -> Self {
        Self {
            nums: HashMap::new(),
            indices: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        self.nums.insert(index, number);
        self.indices
            .entry(number)
            .and_modify(|h| h.push(-index))
            .or_insert(BinaryHeap::from([-index]));
    }

    fn find(&mut self, number: i32) -> i32 {
        if !self.indices.contains_key(&number) {
            return -1;
        }

        while let Some(&i) = self.indices[&number].peek() {
            if self.nums[&-i] != number {
                self.indices.get_mut(&number).unwrap().pop();
            } else {
                return -i;
            }
        }

        -1
    }
}

/**
 * Your NumberContainers object will be instantiated and called as such:
 * let obj = NumberContainers::new();
 * obj.change(index, number);
 * let ret_2: i32 = obj.find(number);
 */
