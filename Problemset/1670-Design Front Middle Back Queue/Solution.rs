use std::collections::VecDeque;

struct FrontMiddleBackQueue {
    front_half: VecDeque<i32>,
    back_half: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrontMiddleBackQueue {
    fn new() -> Self {
        Self {
            front_half: VecDeque::new(),
            back_half: VecDeque::new(),
        }
    }

    fn push_front(&mut self, val: i32) {
        self.front_half.push_front(val);
        if self.front_half.len() > self.back_half.len() {
            self.back_half
                .push_front(self.front_half.pop_back().unwrap());
        }
    }

    fn push_middle(&mut self, val: i32) {
        if self.front_half.len() < self.back_half.len() {
            self.front_half.push_back(val);
        } else {
            self.back_half.push_front(val);
        }
    }

    fn push_back(&mut self, val: i32) {
        self.back_half.push_back(val);
        if self.front_half.len() + 1 < self.back_half.len() {
            self.front_half
                .push_back(self.back_half.pop_front().unwrap());
        }
    }

    fn pop_front(&mut self) -> i32 {
        if self.front_half.len() < self.back_half.len() {
            self.front_half
                .push_back(self.back_half.pop_front().unwrap());
        }
        self.front_half.pop_front().unwrap_or(-1)
    }

    fn pop_middle(&mut self) -> i32 {
        if self.front_half.len() < self.back_half.len() {
            self.back_half.pop_front().unwrap_or(-1)
        } else {
            self.front_half.pop_back().unwrap_or(-1)
        }
    }

    fn pop_back(&mut self) -> i32 {
        if self.front_half.len() == self.back_half.len() {
            self.back_half
                .push_front(self.front_half.pop_back().unwrap_or(-1));
        }
        self.back_half.pop_back().unwrap()
    }
}

/**
 * Your FrontMiddleBackQueue object will be instantiated and called as such:
 * let obj = FrontMiddleBackQueue::new();
 * obj.push_front(val);
 * obj.push_middle(val);
 * obj.push_back(val);
 * let ret_4: i32 = obj.pop_front();
 * let ret_5: i32 = obj.pop_middle();
 * let ret_6: i32 = obj.pop_back();
 */
