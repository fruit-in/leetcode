impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut n = 0;
        let mut x = x - 1;
        while x >= 0 {
            n += 1;
            x -= 2 * n + 1;
        }
        n
    }
}
