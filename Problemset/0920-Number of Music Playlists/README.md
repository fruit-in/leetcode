# 920. Number of Music Playlists
Your music player contains `n` different songs. You want to listen to `goal` songs (not necessarily different) during your trip. To avoid boredom, you will create a playlist so that:

* Every song is played **at least once**.
* A song can only be played again only if `k` other songs have been played.

Given `n`, `goal`, and `k`, return *the number of possible playlists that you can create*. Since the answer can be very large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> n = 3, goal = 3, k = 1
<strong>Output:</strong> 6
<strong>Explanation:</strong> There are 6 possible playlists: [1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], and [3, 2, 1].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 2, goal = 3, k = 0
<strong>Output:</strong> 6
<strong>Explanation:</strong> There are 6 possible playlists: [1, 1, 2], [1, 2, 1], [2, 1, 1], [2, 2, 1], [2, 1, 2], and [1, 2, 2].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 2, goal = 3, k = 1
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are 2 possible playlists: [1, 2, 1] and [2, 1, 2].
</pre>

#### Constraints:
* `0 <= k < n <= goal <= 100`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
        let n = n as usize;
        let goal = goal as usize;
        let k = k as usize;
        let mut dp = vec![vec![0; n + 1]; goal + 1];
        dp[0][0] = 1;

        for i in 0..goal {
            for j in 0..=n.min(i) {
                if j < n {
                    dp[i + 1][j + 1] = (dp[i + 1][j + 1] + dp[i][j] * (n - j)) % 1_000_000_007;
                }
                if j > k {
                    dp[i + 1][j] = (dp[i + 1][j] + dp[i][j] * (j - k)) % 1_000_000_007;
                }
            }
        }

        dp[goal][n] as i32
    }
}
```
