# 2320. 统计放置房子的方式数
一条街道上共有 `n * 2` 个 **地块** ，街道的两侧各有 `n` 个地块。每一边的地块都按从 `1` 到 `n` 编号。每个地块上都可以放置一所房子。

现要求街道同一侧不能存在两所房子相邻的情况，请你计算并返回放置房屋的方式数目。由于答案可能很大，需要对 <code>10<sup>9</sup> + 7</code> 取余后再返回。

注意，如果一所房子放置在这条街某一侧上的第 `i` 个地块，不影响在另一侧的第 `i` 个地块放置房子。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 1
<strong>输出:</strong> 4
<strong>解释:</strong>
可能的放置方式：
1. 所有地块都不放置房子。
2. 一所房子放在街道的某一侧。
3. 一所房子放在街道的另一侧。
4. 放置两所房子，街道两侧各放置一所。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/05/12/arrangements.png)
<pre>
<strong>输入:</strong> n = 2
<strong>输出:</strong> 9
<strong>解释:</strong> 如上图所示，共有 9 种可能的放置方式。
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_house_placements(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![[0, 0]; n];
        dp[0] = [1, 1];

        for i in 1..n {
            dp[i][0] = (dp[i - 1][0] + dp[i - 1][1]) % 1_000_000_007;
            dp[i][1] = dp[i - 1][0];
        }

        (((dp[n - 1][0] + dp[n - 1][1]) as i64).pow(2) % 1_000_000_007) as i32
    }
}
```
