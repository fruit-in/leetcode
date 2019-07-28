impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut n = 1;
        while n <= x / n {
            n += 1;
        }
        n - 1
    }
}
