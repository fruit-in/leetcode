# 808. Soup Servings
There are two types of soup: type A and type B. Initially we have ```N``` ml of each type of soup. There are four kinds of operations:
1. Serve 100 ml of soup A and 0 ml of soup B
2. Serve 75 ml of soup A and 25 ml of soup B
3. Serve 50 ml of soup A and 50 ml of soup B
4. Serve 25 ml of soup A and 75 ml of soup B

When we serve some soup, we give it to someone and we no longer have it.  Each turn, we will choose from the four operations with equal probability 0.25. If the remaining volume of soup is not enough to complete the operation, we will serve as much as we can.  We stop once we no longer have some quantity of both types of soup.

Note that we do not have the operation where all 100 ml's of soup B are used first.

Return the probability that soup A will be empty first, plus half the probability that A and B become empty at the same time.

#### Example:
<pre>
<strong>Input:</strong> N = 50
<strong>Output:</strong> 0.625
<strong>Explanation:</strong>
If we choose the first two operations, A will become empty first. For the third operation, A and B will become empty at the same time. For the fourth operation, B will become empty first. So the total probability of A becoming empty first plus half the probability that A and B become empty at the same time, is 0.25 * (1 + 1 + 0.5 + 0) = 0.625.
</pre>

#### Note:
* ```0 <= N <= 10^9```.
* Answers within ```10^-6``` of the true value will be accepted as correct.

## Solutions (Rust)

### 1. Dynamic Programming
```Rust
impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        let n = (n as usize + 24) / 25;
        if n > 222 {
            return 1_f64;
        }

        let mut dp = vec![vec![0_f64; n + 1]; n + 1];
        dp[0] = vec![1_f64; n + 1];
        dp[0][0] = 0.5;

        for i in 1..=n {
            for j in 1..=n {
                dp[i][j] += dp[i.max(4) - 4][j];
                dp[i][j] += dp[i.max(3) - 3][j - 1];
                dp[i][j] += dp[i.max(2) - 2][j.max(2) - 2];
                dp[i][j] += dp[i - 1][j.max(3) - 3];
                dp[i][j] *= 0.25;
            }
        }

        dp[n][n]
    }
}
```
