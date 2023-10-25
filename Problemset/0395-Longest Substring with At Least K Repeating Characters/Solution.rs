impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        Self::dfs(s.as_bytes(), 0, s.len() - 1, k as usize)
    }

    fn dfs(s: &[u8], l: usize, r: usize, k: usize) -> i32 {
        if l > r || r - l + 1 < k {
            return 0;
        }

        let mut indices = vec![vec![]; 26];
        let mut ret = (r - l + 1) as i32;

        for i in l..=r {
            indices[(s[i] - b'a') as usize].push(i);
        }

        for i in 0..26 {
            if !indices[i].is_empty() && indices[i].len() < k {
                ret = 0;
                ret = ret.max(Self::dfs(s, l, indices[i][0].saturating_sub(1), k));
                for j in 1..indices[i].len() {
                    ret = ret.max(Self::dfs(s, indices[i][j - 1] + 1, indices[i][j] - 1, k));
                }
                ret = ret.max(Self::dfs(s, *indices[i].last().unwrap() + 1, r, k));
                break;
            }
        }

        ret
    }
}
