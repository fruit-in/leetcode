impl Solution {
    pub fn min_flips(target: String) -> i32 {
        target
            .bytes()
            .fold(0, |ret, ch| ret + ((ret % 2) as u8 ^ (ch - b'0')) as i32)
    }
}
