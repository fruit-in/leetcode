# 1473. Paint House III
There is a row of `m` houses in a small city, each house must be painted with one of the `n` colors (labeled from `1` to `n`), some houses that have been painted last summer should not be painted again.

A neighborhood is a maximal group of continuous houses that are painted with the same color.

* For example: `houses = [1,2,2,3,3,2,1,1]` contains `5` neighborhoods `[{1}, {2,2}, {3,3}, {2}, {1,1}]`.

Given an array `houses`, an `m x n` matrix `cost` and an integer `target` where:

* `houses[i]`: is the color of the house `i`, and `0` if the house is not painted yet.
* `cost[i][j]`: is the cost of paint the house `i` with the color `j + 1`.

Return *the minimum cost of painting all the remaining houses in such a way that there are exactly* `target` *neighborhoods*. If it is not possible, return `-1`.

#### Example 1:
<pre>
<strong>Input:</strong> houses = [0,0,0,0,0], cost = [[1,10],[10,1],[10,1],[1,10],[5,1]], m = 5, n = 2, target = 3
<strong>Output:</strong> 9
<strong>Explanation:</strong> Paint houses of this way [1,2,2,1,1]
This array contains target = 3 neighborhoods, [{1}, {2,2}, {1,1}].
Cost of paint all houses (1 + 1 + 1 + 1 + 5) = 9.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> houses = [0,2,1,2,0], cost = [[1,10],[10,1],[10,1],[1,10],[5,1]], m = 5, n = 2, target = 3
<strong>Output:</strong> 11
<strong>Explanation:</strong> Some houses are already painted, Paint the houses of this way [2,2,1,2,2]
This array contains target = 3 neighborhoods, [{2,2}, {1}, {2,2}].
Cost of paint the first and last house (10 + 1) = 11.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> houses = [3,1,2,3], cost = [[1,1,1],[1,1,1],[1,1,1],[1,1,1]], m = 4, n = 3, target = 3
<strong>Output:</strong> -1
<strong>Explanation:</strong> Houses are already painted with a total of 4 neighborhoods [{3},{1},{2},{3}] different of target = 3.
</pre>

#### Constraints:
* `m == houses.length == cost.length`
* `n == cost[i].length`
* `1 <= m <= 100`
* `1 <= n <= 20`
* `1 <= target <= m`
* `0 <= houses[i] <= n`
* <code>1 <= cost[i][j] <= 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        let (m, n, target) = (m as usize, n as usize, target as usize);
        let houses = houses.into_iter().map(|x| x as usize).collect::<Vec<_>>();
        let mut dp = vec![vec![vec![None; n]; target + 1]; m];

        if houses[0] > 0 {
            dp[0][1][houses[0] as usize - 1] = Some(0);
        } else {
            for j in 0..n {
                dp[0][1][j] = Some(cost[0][j]);
            }
        }

        for i in 1..m {
            for k in 1..=target {
                for j in 0..n {
                    if let Some(x) = dp[i - 1][k][j] {
                        if houses[i] > 0 {
                            if houses[i] - 1 == j {
                                dp[i][k][j] = Some(dp[i][k][j].unwrap_or(i32::MAX).min(x));
                            } else if k + 1 <= target {
                                dp[i][k + 1][houses[i] - 1] =
                                    Some(dp[i][k + 1][houses[i] - 1].unwrap_or(i32::MAX).min(x));
                            }
                        } else {
                            for jj in 0..n {
                                if jj == j {
                                    dp[i][k][jj] =
                                        Some(dp[i][k][jj].unwrap_or(i32::MAX).min(x + cost[i][jj]));
                                } else if k + 1 <= target {
                                    dp[i][k + 1][jj] = Some(
                                        dp[i][k + 1][jj].unwrap_or(i32::MAX).min(x + cost[i][jj]),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        dp[m - 1][target]
            .iter()
            .filter_map(|&x| x)
            .min()
            .unwrap_or(-1)
    }
}
```
