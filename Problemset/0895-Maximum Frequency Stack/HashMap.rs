use std::collections::HashMap;

struct FreqStack {
    num_freq: HashMap<i32, i32>,
    freq_stack: HashMap<i32, Vec<i32>>,
    max_freq: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {

    fn new() -> Self {
        Self {
            num_freq: HashMap::new(),
            freq_stack: HashMap::new(),
            max_freq: 0,
        }
    }

    fn push(&mut self, x: i32) {
        let freq = self.num_freq.entry(x).or_default();
        *freq += 1;
        self.freq_stack.entry(*freq).or_default().push(x);
        self.max_freq = self.max_freq.max(*freq);
    }

    fn pop(&mut self) -> i32 {
        let stack = self.freq_stack.get_mut(&self.max_freq).unwrap();
        let x = stack.pop().unwrap();
        *self.num_freq.get_mut(&x).unwrap() -= 1;
        if stack.is_empty() {
            self.max_freq -= 1;
        }
        x
    }
}

/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 */
