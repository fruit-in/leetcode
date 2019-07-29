struct MyCircularQueue {
    data: Vec<i32>,
    size: usize,
    len: usize,
    head: usize,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {

    /** Initialize your data structure here. Set the size of the queue to be k. */
    fn new(k: i32) -> Self {
        let k = k as usize;
        Self {
            data: vec![0; k],
            size: k,
            len: 0,
            head: 0,
        }
    }
    
    /** Insert an element into the circular queue. Return true if the operation is successful. */
    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.data[(self.head + self.len) % self.size] = value;
            self.len += 1;
            true
        }
    }
    
    /** Delete an element from the circular queue. Return true if the operation is successful. */
    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.head += 1;
            self.head %= self.size;
            self.len -= 1;
            true
        }
    }
    
    /** Get the front item from the queue. */
    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.head]
        }
    }
    
    /** Get the last item from the queue. */
    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[(self.head + self.len - 1) % self.size]
        }
    }
    
    /** Checks whether the circular queue is empty or not. */
    fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    /** Checks whether the circular queue is full or not. */
    fn is_full(&self) -> bool {
        self.len == self.size
    }
}

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */
