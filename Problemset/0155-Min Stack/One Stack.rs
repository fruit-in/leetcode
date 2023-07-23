struct MinStack {
    data: Vec<(i32, i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }
    
    fn push(&mut self, x: i32) {
        if self.data.len() == 0 || x <= self.get_min() {
            self.data.push((x, x));
        } else {
            self.data.push((x, self.get_min()));
        }
    }
    
    fn pop(&mut self) {
        self.data.pop();
    }
    
    fn top(&self) -> i32 {
        self.data.last().unwrap().0
    }
    
    fn get_min(&self) -> i32 {
        self.data.last().unwrap().1
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
