impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count0 = (b'1' - s[0]) as i32;
        let mut count1 = 0;
        let mut ret = 0;

        for i in 1..s.len() {
            if s[i] == b'0' && s[i - 1] == b'1' {
                count0 = 0;
                count1 = 0;
            }

            if s[i] == b'0' {
                count0 += 1;
            } else if s[i] == b'1' {
                count1 += 1;
            }

            ret = ret.max(count0.min(count1) * 2);
        }

        ret
    }
}
