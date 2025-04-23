# 1434. Number of Ways to Wear Different Hats to Each Other
There are `n` people and `40` types of hats labeled from `1` to `40`.

Given a 2D integer array `hats`, where `hats[i]` is a list of all hats preferred by the <code>i<sup>th</sup></code> person.

Return the number of ways that `n` people can wear **different** hats from each other.

Since the answer may be too large, return it modulo <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> hats = [[3,4],[4,5],[5]]
<strong>Output:</strong> 1
<strong>Explanation:</strong> There is only one way to choose hats given the conditions.
First person choose hat 3, Second person choose hat 4 and last one hat 5.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> hats = [[3,5,1],[3,5]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> There are 4 ways to choose hats:
(3,5), (5,3), (1,3) and (1,5)
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> hats = [[1,2,3,4],[1,2,3,4],[1,2,3,4],[1,2,3,4]]
<strong>Output:</strong> 24
<strong>Explanation:</strong> Each person can choose hats labeled from 1 to 4.
Number of Permutations of (1,2,3,4) = 24.
</pre>

#### Constraints:
* `n == hats.length`
* `1 <= n <= 10`
* `1 <= hats[i].length <= 40`
* `1 <= hats[i][j] <= 40`
* `hats[i]` contains a list of **unique** integers.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        let n = hats.len();
        let mut dp = vec![vec![0; 1 << n]; 41];
        dp[0][0] = 1;

        for i in 1..=40 {
            dp[i] = dp[i - 1].clone();

            for j in 0..n {
                if !hats[j].contains(&(i as i32)) {
                    continue;
                }

                for k in 0..(1 << n) {
                    if (k >> j) & 1 == 0 {
                        dp[i][k | (1 << j)] = (dp[i][k | (1 << j)] + dp[i - 1][k]) % 1_000_000_007;
                    }
                }
            }
        }

        dp[40][(1 << n) - 1]
    }
}
```
