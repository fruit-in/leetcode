struct RecentCounter {
    time: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        Self {
            time: Vec::new(),
        }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.time.push(t);

        while t - self.time[0] > 3000 {
            self.time.remove(0);
        }

        self.time.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
