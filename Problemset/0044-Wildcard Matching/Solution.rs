impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;

        for i in 1..=p.len() {
            let mut tmp = vec![false; s.len() + 1];
            let mut prefix_or = dp[0];

            if p[i - 1] == b'*' {
                tmp[0] = dp[0];
            }
            for j in 1..=s.len() {
                prefix_or |= dp[j];
                match p[i - 1] {
                    b'?' => tmp[j] = dp[j - 1],
                    b'*' => tmp[j] = prefix_or,
                    _ => tmp[j] = p[i - 1] == s[j - 1] && dp[j - 1],
                }
            }

            dp = tmp;
        }

        dp[s.len()]
    }
}
