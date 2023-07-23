# 2400. 恰好移动 k 步到达某一位置的方法数目
给你两个 **正** 整数 `startPos` 和 `endPos` 。最初，你站在 **无限** 数轴上位置 `startPos` 处。在一步移动中，你可以向左或者向右移动一个位置。

给你一个正整数 `k` ，返回从 `startPos` 出发、**恰好** 移动 `k` 步并到达 `endPos` 的 **不同** 方法数目。由于答案可能会很大，返回对 <code>10<sup>9</sup> + 7</code> **取余** 的结果。

如果所执行移动的顺序不完全相同，则认为两种方法不同。

**注意：**数轴包含负整数。

#### 示例 1:
<pre>
<strong>输入:</strong> startPos = 1, endPos = 2, k = 3
<strong>输出:</strong> 3
<strong>解释:</strong> 存在 3 种从 1 到 2 且恰好移动 3 步的方法：
- 1 -> 2 -> 3 -> 2.
- 1 -> 2 -> 1 -> 2.
- 1 -> 0 -> 1 -> 2.
可以证明不存在其他方法，所以返回 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> startPos = 2, endPos = 5, k = 10
<strong>输出:</strong> 0
<strong>解释:</strong> 不存在从 2 到 5 且恰好移动 10 步的方法。
</pre>

#### 提示:
* `1 <= startPos, endPos, k <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn number_of_ways(start_pos: i32, end_pos: i32, k: i32) -> i32 {
        if start_pos - k > end_pos || start_pos + k < end_pos {
            return 0;
        }

        let mut dp = vec![vec![0; 2 * k as usize + 1]; k as usize + 1];
        dp[0][k as usize] = 1;

        for i in 0..k as usize {
            for j in k as usize - i..=k as usize + i {
                dp[i + 1][j - 1] = (dp[i + 1][j - 1] + dp[i][j]) % 1_000_000_007;
                dp[i + 1][j + 1] = (dp[i + 1][j + 1] + dp[i][j]) % 1_000_000_007;
            }
        }

        dp[k as usize][(end_pos - start_pos + k) as usize]
    }
}
```
