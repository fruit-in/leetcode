impl Solution {
    pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
        let text = text.as_bytes();
        let pattern = pattern.as_bytes();
        let n = text.len();
        let mut count0 = 1;
        let mut count1 = 1;
        let mut ret0 = 0;
        let mut ret1 = 0;

        for i in 0..text.len() {
            if text[i] == pattern[1] {
                ret0 += count0;
            }
            if text[i] == pattern[0] {
                count0 += 1;
            }

            if text[n - 1 - i] == pattern[0] {
                ret1 += count1;
            }
            if text[n - 1 - i] == pattern[1] {
                count1 += 1;
            }
        }

        ret0.max(ret1)
    }
}
