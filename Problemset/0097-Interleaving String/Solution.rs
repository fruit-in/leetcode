impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        let mut dp = vec![false; s2.len() + 1];

        for i in 0..=s1.len() {
            let mut tmp = vec![false; s2.len() + 1];
            tmp[0] = i == 0;

            for j in 0..=s2.len() {
                tmp[j] |= i > 0 && s1[i - 1] == s3[i + j - 1] && dp[j];
                tmp[j] |= j > 0 && s2[j - 1] == s3[i + j - 1] && tmp[j - 1];
            }

            dp = tmp;
        }

        dp[s2.len()]
    }
}
