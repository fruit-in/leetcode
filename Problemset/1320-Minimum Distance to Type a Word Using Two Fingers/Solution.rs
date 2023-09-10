impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        let word = word
            .bytes()
            .map(|b| (b - b'A') as usize)
            .collect::<Vec<_>>();
        let mut xy = (0..26).map(|i: i32| (i / 6, i % 6)).collect::<Vec<_>>();
        let mut dp = vec![vec![vec![i32::MAX; 26]; 26]; word.len() + 1];
        let mut ret = i32::MAX;
        dp[0] = vec![vec![0; 26]; 26];

        for i in 0..word.len() {
            let (x1, y1) = xy[word[i]];

            for j in 0..26 {
                let (x2, y2) = xy[j];

                for k in 0..26 {
                    if dp[i][j][k] == i32::MAX {
                        continue;
                    }

                    let (x3, y3) = xy[k];

                    dp[i + 1][word[i]][k] =
                        dp[i + 1][word[i]][k].min(dp[i][j][k] + (x1 - x2).abs() + (y1 - y2).abs());
                    dp[i + 1][j][word[i]] =
                        dp[i + 1][j][word[i]].min(dp[i][j][k] + (x1 - x3).abs() + (y1 - y3).abs());
                }
            }
        }

        for i in 0..26 {
            ret = ret.min(dp[word.len()][*word.last().unwrap()][i]);
            ret = ret.min(dp[word.len()][i][*word.last().unwrap()]);
        }

        ret
    }
}
