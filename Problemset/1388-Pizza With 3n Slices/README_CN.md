# 1388. 3n 块披萨
给你一个披萨，它由 3n 块不同大小的部分组成，现在你和你的朋友们需要按照如下规则来分披萨：
* 你挑选 **任意** 一块披萨。
* Alice 将会挑选你所选择的披萨逆时针方向的下一块披萨。
* Bob 将会挑选你所选择的披萨顺时针方向的下一块披萨。
* 重复上述过程直到没有披萨剩下。

每一块披萨的大小按顺时针方向由循环数组 `slices` 表示。

请你返回你可以获得的披萨大小总和的最大值。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/02/18/sample_3_1723.png)
<pre>
<strong>输入:</strong> slices = [1,2,3,4,5,6]
<strong>输出:</strong> 10
<strong>解释:</strong> 选择大小为 4 的披萨，Alice 和 Bob 分别挑选大小为 3 和 5 的披萨。然后你选择大小为 6 的披萨，Alice 和 Bob 分别挑选大小为 2 和 1 的披萨。你获得的披萨总大小为 4 + 6 = 10 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/02/18/sample_4_1723.png)
<pre>
<strong>输入:</strong> slices = [8,9,8,6,1,1]
<strong>输出:</strong> 16
<strong>解释:</strong> 两轮都选大小为 8 的披萨。如果你选择大小为 9 的披萨，你的朋友们就会选择大小为 8 的披萨，这种情况下你的总和不是最大的。
</pre>

#### 提示:
* `1 <= slices.length <= 500`
* `slices.length % 3 == 0`
* `1 <= slices[i] <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        let n = slices.len();
        let m = n / 3;
        let mut dp = vec![vec![0; m + 1]; n];

        dp[1][1] = slices[1];
        for i in 2..n {
            for j in 1..=m.min(i / 2 + 1) {
                dp[i][j] = dp[i - 1][j].max(dp[i - 2][j - 1] + slices[i]);
            }
        }

        dp[0][1] = slices[0];
        dp[1][1] = slices[0].max(slices[1]);
        for i in 2..n - 1 {
            for j in 1..=m.min(i / 2 + 1) {
                dp[i][j] = dp[i - 1][j].max(dp[i - 2][j - 1] + slices[i]);
            }
        }

        dp[n - 1][m].max(dp[n - 2][m])
    }
}
```
