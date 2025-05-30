# 1388. Pizza With 3n Slices
There is a pizza with `3n` slices of varying size, you and your friends will take slices of pizza as follows:
* You will pick **any** pizza slice.
* Your friend Alice will pick the next slice in the anti-clockwise direction of your pick.
* Your friend Bob will pick the next slice in the clockwise direction of your pick.
* Repeat until there are no more slices of pizzas.

Given an integer array `slices` that represent the sizes of the pizza slices in a clockwise direction, return *the maximum possible sum of slice sizes that you can pick*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/02/18/sample_3_1723.png)
<pre>
<strong>Input:</strong> slices = [1,2,3,4,5,6]
<strong>Output:</strong> 10
<strong>Explanation:</strong> Pick pizza slice of size 4, Alice and Bob will pick slices with size 3 and 5 respectively. Then Pick slices with size 6, finally Alice and Bob will pick slice of size 2 and 1 respectively. Total = 4 + 6.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/02/18/sample_4_1723.png)
<pre>
<strong>Input:</strong> slices = [8,9,8,6,1,1]
<strong>Output:</strong> 16
<strong>Explanation:</strong> Pick pizza slice of size 8 in each turn. If you pick slice with size 9 your partners will pick slices of size 8.
</pre>

#### Constraints:
* `3 * n == slices.length`
* `1 <= slices.length <= 500`
* `1 <= slices[i] <= 1000`

## Solutions (Rust)

### 1. Solution
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
