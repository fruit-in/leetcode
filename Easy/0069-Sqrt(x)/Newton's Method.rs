impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let x = x as usize;
        let mut n = x;
        while n > x / n {
            n = (n + x / n) / 2;
        }
        n as i32
    }
}
