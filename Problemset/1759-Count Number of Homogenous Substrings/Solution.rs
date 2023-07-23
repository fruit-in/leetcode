impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let s = s.as_bytes();
        let mut count = 1;
        let mut ret = 1;

        for i in 1..s.len() {
            if s[i] != s[i - 1] {
                count = 0;
            }

            count += 1;
            ret = (ret + count) % 1_000_000_007;
        }

        ret
    }
}
