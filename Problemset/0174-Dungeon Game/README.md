# 174. Dungeon Game
The demons had captured the princess and imprisoned her in **the bottom-right corner** of a `dungeon`. The `dungeon` consists of `m x n` rooms laid out in a 2D grid. Our valiant knight was initially positioned in **the top-left room** and must fight his way through `dungeon` to rescue the princess.

The knight has an initial health point represented by a positive integer. If at any point his health point drops to `0` or below, he dies immediately.

Some of the rooms are guarded by demons (represented by negative integers), so the knight loses health upon entering these rooms; other rooms are either empty (represented as 0) or contain magic orbs that increase the knight's health (represented by positive integers).

To reach the princess as quickly as possible, the knight decides to move only **rightward** or **downward** in each step.

Return *the knight's minimum initial health so that he can rescue the princess*.

**Note** that any room can contain threats or power-ups, even the first room the knight enters and the bottom-right room where the princess is imprisoned.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/03/13/dungeon-grid-1.jpg)
<pre>
<strong>Input:</strong> dungeon = [[-2,-3,3],[-5,-10,1],[10,30,-5]]
<strong>Output:</strong> 7
<strong>Explanation:</strong> The initial health of the knight must be at least 7 if he follows the optimal path: RIGHT-> RIGHT -> DOWN -> DOWN.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> dungeon = [[0]]
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `m == dungeon.length`
* `n == dungeon[i].length`
* `1 <= m, n <= 200`
* `-1000 <= dungeon[i][j] <= 1000`

## Solutions (Rust)

### 1. Solution
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
