use std::collections::VecDeque;

struct DataStream {
    deque: VecDeque<i32>,
    value: i32,
    k: usize,
    count: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DataStream {
    fn new(value: i32, k: i32) -> Self {
        Self {
            deque: VecDeque::with_capacity(k as usize),
            value,
            k: k as usize,
            count: 0,
        }
    }

    fn consec(&mut self, num: i32) -> bool {
        if self.deque.len() == self.k && self.deque.pop_front().unwrap() == self.value {
            self.count -= 1;
        }

        self.deque.push_back(num);

        if num == self.value {
            self.count += 1;
        }

        self.count == self.k
    }
}

/**
 * Your DataStream object will be instantiated and called as such:
 * let obj = DataStream::new(value, k);
 * let ret_1: bool = obj.consec(num);
 */
