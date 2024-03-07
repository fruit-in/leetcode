# 920. 播放列表的数量
你的音乐播放器里有 `n` 首不同的歌，在旅途中，你计划听 `goal` 首歌（不一定不同，即，允许歌曲重复）。你将会按如下规则创建播放列表：

* 每首歌 **至少播放一次** 。
* 一首歌只有在其他 `k` 首歌播放完之后才能再次播放。

给你 `n`、`goal` 和 `k` ，返回可以满足要求的播放列表的数量。由于答案可能非常大，请返回对 <code>10<sup>9</sup> + 7</code> **取余** 的结果。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 3, goal = 3, k = 1
<strong>输出:</strong> 6
<strong>解释:</strong> 有 6 种可能的播放列表。[1, 2, 3]，[1, 3, 2]，[2, 1, 3]，[2, 3, 1]，[3, 1, 2]，[3, 2, 1] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 2, goal = 3, k = 0
<strong>输出:</strong> 6
<strong>解释:</strong> 有 6 种可能的播放列表。[1, 1, 2]，[1, 2, 1]，[2, 1, 1]，[2, 2, 1]，[2, 1, 2]，[1, 2, 2] 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 2, goal = 3, k = 1
<strong>输出:</strong> 2
<strong>解释:</strong> 有 2 种可能的播放列表。[1, 2, 1]，[2, 1, 2] 。
</pre>

#### 提示:
* `0 <= k < n <= goal <= 100`

## 题解 (Rust)

### 1. 题解
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
