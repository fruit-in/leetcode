struct MyCircularDeque {
    deque: Vec<i32>,
    size: usize,
    front: usize,
    back: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            deque: vec![0; k as usize],
            size: 0,
            front: 0,
            back: 0,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        if !self.is_empty() {
            self.front = (self.front + self.deque.len() - 1) % self.deque.len();
        }
        self.deque[self.front] = value;
        self.size += 1;

        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        if !self.is_empty() {
            self.back = (self.back + 1) % self.deque.len();
        }
        self.deque[self.back] = value;
        self.size += 1;

        true
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        if self.size > 1 {
            self.front = (self.front + 1) % self.deque.len();
        }
        self.size -= 1;

        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        if self.size > 1 {
            self.back = (self.back + self.deque.len() - 1) % self.deque.len();
        }
        self.size -= 1;

        true
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }

        self.deque[self.front]
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }

        self.deque[self.back]
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.deque.len()
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */
