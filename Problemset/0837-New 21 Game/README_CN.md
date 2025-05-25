# 837. 新 21 点
爱丽丝参与一个大致基于纸牌游戏 **“21点”** 规则的游戏，描述如下：

爱丽丝以 `0` 分开始，并在她的得分少于 `k` 分时抽取数字。 抽取时，她从 `[1, maxPts]` 的范围中随机获得一个整数作为分数进行累计，其中 `maxPts` 是一个整数。 每次抽取都是独立的，其结果具有相同的概率。

当爱丽丝获得 `k` 分 **或更多分** 时，她就停止抽取数字。

爱丽丝的分数不超过 `n` 的概率是多少？

与实际答案误差不超过 <code>10<sup>-5</sup></code> 的答案将被视为正确答案。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 10, k = 1, maxPts = 10
<strong>输出:</strong> 1.00000
<strong>解释:</strong> 爱丽丝得到一张牌，然后停止。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 6, k = 1, maxPts = 10
<strong>输出:</strong> 0.60000
<strong>解释:</strong> 爱丽丝得到一张牌，然后停止。 在 10 种可能性中的 6 种情况下，她的得分不超过 6 分。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 21, k = 17, maxPts = 10
<strong>输出:</strong> 0.73278
</pre>

#### 提示:
* <code>0 <= k <= n <= 10<sup>4</sup></code>
* <code>1 <= maxPts <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k - 1 + max_pts <= n || k == 0 {
            return 1.;
        }

        let (n, k, max_pts) = (n as usize, k as usize, max_pts as usize);
        let mut window_sum = 1.;
        let mut dp = vec![0.; n + 1];
        let mut ret = 0.;
        dp[0] = 1.;

        for i in 1..=n {
            dp[i] = window_sum / max_pts as f64;
            if i >= max_pts {
                window_sum -= dp[i - max_pts];
            }
            if i >= k {
                ret += dp[i];
            } else {
                window_sum += dp[i];
            }
        }

        ret
    }
}
```
