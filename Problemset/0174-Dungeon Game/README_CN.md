# 174. 地下城游戏
恶魔们抓住了公主并将她关在了地下城 `dungeon` 的 **右下角** 。地下城是由 `m x n` 个房间组成的二维网格。我们英勇的骑士最初被安置在 **左上角** 的房间里，他必须穿过地下城并通过对抗恶魔来拯救公主。

骑士的初始健康点数为一个正整数。如果他的健康点数在某一时刻降至 0 或以下，他会立即死亡。

有些房间由恶魔守卫，因此骑士在进入这些房间时会失去健康点数（若房间里的值为*负整数*，则表示骑士将损失健康点数）；其他房间要么是空的（房间里的值为 *0*），要么包含增加骑士健康点数的魔法球（若房间里的值为*正整数*，则表示骑士将增加健康点数）。

为了尽快解救公主，骑士决定每次只 **向右** 或 **向下** 移动一步。

返回确保骑士能够拯救到公主所需的最低初始健康点数。

**注意：**任何房间都可能对骑士的健康点数造成威胁，也可能增加骑士的健康点数，包括骑士进入的左上角房间以及公主被监禁的右下角房间。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/03/13/dungeon-grid-1.jpg)
<pre>
<strong>输入:</strong> dungeon = [[-2,-3,3],[-5,-10,1],[10,30,-5]]
<strong>输出:</strong> 7
<strong>解释:</strong> 如果骑士遵循最佳路径：右 -> 右 -> 下 -> 下 ，则骑士的初始健康点数至少为 7 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> dungeon = [[0]]
<strong>输出:</strong> 1
</pre>

#### 提示:
* `m == dungeon.length`
* `n == dungeon[i].length`
* `1 <= m, n <= 200`
* `-1000 <= dungeon[i][j] <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (dungeon.len(), dungeon[0].len());
        let mut dp = vec![vec![0; n]; m];

        for r in (0..m).rev() {
            for c in (0..n).rev() {
                if r < m - 1 && c < n - 1 {
                    dp[r][c] = 1.max(dp[r + 1][c].min(dp[r][c + 1]) - dungeon[r][c]);
                } else if r < m - 1 {
                    dp[r][c] = 1.max(dp[r + 1][c] - dungeon[r][c]);
                } else if c < n - 1 {
                    dp[r][c] = 1.max(dp[r][c + 1] - dungeon[r][c]);
                } else {
                    dp[r][c] = 1.max(1 - dungeon[r][c]);
                }
            }
        }

        dp[0][0]
    }
}
```
