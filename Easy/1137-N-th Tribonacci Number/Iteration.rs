impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        match n {
            0 => 0,
            1 | 2 => 1,
            _ => {
                let mut t = (0, 1, 1);
                for i in 3..=n {
                    t = (t.1, t.2, t.0 + t.1 + t.2);
                }
                t.2
            },
        }
    }
}
