impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let s = s.as_bytes();
        let mut i = 0;
        let mut ret = 0;

        while i < s.len() {
            if s[i] == b'X' {
                i += 2;
                ret += 1;
            }
            i += 1;
        }

        ret
    }
}
