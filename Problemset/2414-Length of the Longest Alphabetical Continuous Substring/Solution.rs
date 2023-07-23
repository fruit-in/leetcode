impl Solution {
    pub fn longest_continuous_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count = 1;
        let mut ret = 1;

        for i in 1..s.len() {
            if s[i - 1] + 1 == s[i] {
                count += 1;
                ret = ret.max(count);
            } else {
                count = 1;
            }
        }

        ret
    }
}
