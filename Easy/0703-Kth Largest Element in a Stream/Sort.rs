struct KthLargest {
    k: usize,
    elements: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        Self {
            k: k as usize,
            elements: nums,
        }
    }

    fn add(&mut self, val: i32) -> i32 {
        self.elements.push(val);
        self.elements.sort_unstable_by(|a, b| b.cmp(a));
        self.elements[self.k - 1]
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
