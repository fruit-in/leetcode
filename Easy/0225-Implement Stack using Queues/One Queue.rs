struct MyStack {
    queue: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            queue: Vec::new(),
        }
    }
    
    /** Push element x onto stack. */
    fn push(&mut self, x: i32) {
        self.queue.push(x);
        for _ in 1..self.queue.len() {
            let temp = self.queue.remove(0);
            self.queue.push(temp)
        }
    }
    
    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        self.queue.remove(0)
    }
    
    /** Get the top element. */
    fn top(&self) -> i32 {
        *self.queue.first().unwrap()
    }
    
    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */
