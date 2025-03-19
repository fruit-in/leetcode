impl Solution {
    pub fn check_partitioning(s: String) -> bool {
        let s = s.as_bytes();
        let mut is_palindrome = vec![vec![false; s.len()]; s.len()];

        for r in 0..s.len() {
            for l in 0..=r {
                is_palindrome[l][r] = s[l] == s[r];
                if is_palindrome[l][r] && l + 2 < r {
                    is_palindrome[l][r] = is_palindrome[l + 1][r - 1];
                }
            }
        }

        for l in 1..s.len() - 1 {
            for r in l..s.len() - 1 {
                if is_palindrome[0][l - 1]
                    && is_palindrome[l][r]
                    && is_palindrome[r + 1][s.len() - 1]
                {
                    return true;
                }
            }
        }

        false
    }
}
