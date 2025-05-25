# 837. New 21 Game
Alice plays the following game, loosely based on the card game **"21"**.

Alice starts with `0` points and draws numbers while she has less than `k` points. During each draw, she gains an integer number of points randomly from the range `[1, maxPts]`, where `maxPts` is an integer. Each draw is independent and the outcomes have equal probabilities.

Alice stops drawing numbers when she gets `k` **or more points**.

Return the probability that Alice has `n` or fewer points.

Answers within <code>10<sup>-5</sup></code> of the actual answer are considered accepted.

#### Example 1:
<pre>
<strong>Input:</strong> n = 10, k = 1, maxPts = 10
<strong>Output:</strong> 1.00000
<strong>Explanation:</strong> Alice gets a single card, then stops.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 6, k = 1, maxPts = 10
<strong>Output:</strong> 0.60000
<strong>Explanation:</strong> Alice gets a single card, then stops.
In 6 out of 10 possibilities, she is at or below 6 points.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 21, k = 17, maxPts = 10
<strong>Output:</strong> 0.73278
</pre>

#### Constraints:
* <code>0 <= k <= n <= 10<sup>4</sup></code>
* <code>1 <= maxPts <= 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
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
