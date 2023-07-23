impl Solution {
    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        let m = a.max(b).max(c);
        let s = a + b + c;

        if s <= 2 * m {
            s - m
        } else {
            s / 2
        }
    }
}
