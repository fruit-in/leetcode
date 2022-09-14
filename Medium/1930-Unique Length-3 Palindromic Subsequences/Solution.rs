impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ret = 0;

        for c in b'a'..=b'z' {
            let mut mask = 0_i32;

            for i in s.iter().position(|&x| x == c).unwrap_or(0) + 1
                ..s.iter().rposition(|&x| x == c).unwrap_or(0)
            {
                mask |= 1 << (s[i] - b'a');
            }

            ret += mask.count_ones();
        }

        ret as i32
    }
}
