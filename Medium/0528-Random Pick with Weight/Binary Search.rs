use rand::{thread_rng, Rng};

struct Solution {
    prefix_sum: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(mut w: Vec<i32>) -> Self {
        for i in 1..w.len() {
            w[i] += w[i - 1];
        }

        Self { prefix_sum: w }
    }

    fn pick_index(&self) -> i32 {
        let x = thread_rng().gen_range(1, self.prefix_sum.last().unwrap() + 1);

        match self.prefix_sum.binary_search(&x) {
            Ok(i) => i as i32,
            Err(i) => i as i32,
        }
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(w);
 * let ret_1: i32 = obj.pick_index();
 */
