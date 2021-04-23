impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut b = b'0';
        let mut count = 0;

        for c in s.bytes() {
            if c != b {
                count += 1;
            }
            b = b'1' - b + b'0';
        }

        count.min(s.len() - count) as i32
    }
}
