use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct DinnerPlates {
    capacity: usize,
    stacks: Vec<Vec<i32>>,
    non_full: BinaryHeap<Reverse<usize>>,
    non_empty: BinaryHeap<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            stacks: vec![],
            non_full: BinaryHeap::new(),
            non_empty: BinaryHeap::new(),
        }
    }

    fn push(&mut self, val: i32) {
        while let Some(&Reverse(i)) = self.non_full.peek() {
            if self.stacks[i].len() == self.capacity {
                self.non_full.pop();
            } else {
                break;
            }
        }

        if self.non_full.is_empty() {
            self.non_full.push(Reverse(self.stacks.len()));
            self.stacks.push(Vec::with_capacity(self.capacity));
        }

        if let Some(&Reverse(i)) = self.non_full.peek() {
            if self.stacks[i].is_empty() {
                self.non_empty.push(i);
            }
            self.stacks[i].push(val);
        }
    }

    fn pop(&mut self) -> i32 {
        while let Some(&i) = self.non_empty.peek() {
            if self.stacks[i].is_empty() {
                self.non_empty.pop();
            } else {
                break;
            }
        }

        if self.non_empty.is_empty() {
            return -1;
        }

        self.pop_at_stack(*self.non_empty.peek().unwrap() as i32)
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let index = index as usize;

        if self.stacks.len() <= index || self.stacks[index].is_empty() {
            return -1;
        }

        if self.stacks[index].len() == self.capacity {
            self.non_full.push(Reverse(index));
        }
        self.stacks[index].pop().unwrap()
    }
}

/**
 * Your DinnerPlates object will be instantiated and called as such:
 * let obj = DinnerPlates::new(capacity);
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.pop_at_stack(index);
 */
