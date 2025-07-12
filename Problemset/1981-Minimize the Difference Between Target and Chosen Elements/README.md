# 1981. Minimize the Difference Between Target and Chosen Elements
You are given an `m x n` integer matrix `mat` and an integer `target`.

Choose one integer from **each row** in the matrix such that the **absolute difference** between `target` and the **sum** of the chosen elements is **minimized**.

Return *the **minimum absolute difference***.

The **absolute difference** between two numbers `a` and `b` is the absolute value of `a - b`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/08/03/matrix1.png)
<pre>
<strong>Input:</strong> mat = [[1,2,3],[4,5,6],[7,8,9]], target = 13
<strong>Output:</strong> 0
<strong>Explanation:</strong> One possible choice is to:
- Choose 1 from the first row.
- Choose 5 from the second row.
- Choose 7 from the third row.
The sum of the chosen elements is 13, which equals the target, so the absolute difference is 0.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/08/03/matrix1-1.png)
<pre>
<strong>Input:</strong> mat = [[1],[2],[3]], target = 100
<strong>Output:</strong> 94
<strong>Explanation:</strong> The best possible choice is to:
- Choose 1 from the first row.
- Choose 2 from the second row.
- Choose 3 from the third row.
The sum of the chosen elements is 6, and the absolute difference is 94.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/08/03/matrix1-3.png)
<pre>
<strong>Input:</strong> mat = [[1,2,9,8,7]], target = 6
<strong>Output:</strong> 1
<strong>Explanation:</strong> The best choice is to choose 7 from the first row.
The absolute difference is 1.
</pre>

#### Constraints:
* `m == mat.length`
* `n == mat[i].length`
* `1 <= m, n <= 70`
* `1 <= mat[i][j] <= 70`
* `1 <= target <= 800`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimize_the_difference(mat: Vec<Vec<i32>>, target: i32) -> i32 {
        let target = target as usize;
        let mut dp = vec![i32::MAX; target + 1];
        dp[0] = 0;

        for i in 0..mat.len() {
            let mut tmp = vec![i32::MAX; target + 1];

            for j in 0..mat[0].len() {
                for k in 0..=target {
                    tmp[target.min(k + mat[i][j] as usize)] = tmp
                        [target.min(k + mat[i][j] as usize)]
                    .min(dp[k].saturating_add(mat[i][j]));
                }
            }

            dp = tmp;
        }

        dp.iter().map(|&x| (x - target as i32).abs()).min().unwrap()
    }
}
```
