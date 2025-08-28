impl Solution {
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        let n = lcp.len();
        let mut masks = vec![0_i32; n];
        let mut word = vec![b'a'; n];

        for i in 0..n {
            if lcp[i][i] as usize != n - i || masks[i].trailing_ones() > 25 {
                return "".to_string();
            }

            word[i] += masks[i].trailing_ones() as u8;

            for j in i + 1..n {
                if lcp[i][j] > 0 && i + 1 < n && j + 1 < n && lcp[i + 1][j + 1] != lcp[i][j] - 1 {
                    return "".to_string();
                } else if lcp[i][j] != lcp[j][i] {
                    return "".to_string();
                } else if lcp[i][j] as usize > n - i.max(j) {
                    return "".to_string();
                }

                masks[j] |= match lcp[i][j] {
                    0 => 1 << (word[i] - b'a'),
                    _ => i32::MAX ^ (1 << (word[i] - b'a')),
                };
            }
        }

        String::from_utf8(word).unwrap()
    }
}
