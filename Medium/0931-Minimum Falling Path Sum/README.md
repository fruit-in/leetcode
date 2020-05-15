# 931. Minimum Falling Path Sum
Given a **square** array of integers `A`, we want the **minimum** sum of a *falling path* through `A`.

A falling path starts at any element in the first row, and chooses one element from each row.  The next row's choice must be in a column that is different from the previous row's column by at most one.

#### Example 1:
<pre>
<strong>Input:</strong> [[1,2,3],[4,5,6],[7,8,9]]
<strong>Output:</strong> 12
<strong>Explanation:</strong>
The possible falling paths are:
<ul>
<li>[1,4,7], [1,4,8], [1,5,7], [1,5,8], [1,5,9]</li>
<li>[2,4,7], [2,4,8], [2,5,7], [2,5,8], [2,5,9], [2,6,8], [2,6,9]</li>
<li>[3,5,7], [3,5,8], [3,5,9], [3,6,8], [3,6,9]</li>
</ul>
The falling path with the smallest sum is [1,4,7], so the answer is 12.
</pre>

#### Note:
1. `1 <= A.length == A[0].length <= 100`
2. `-100 <= A[i][j] <= 100`

## Solutions (Rust)

### 1. Dynamic Programming
```Rust
impl Solution {
    pub fn min_falling_path_sum(a: Vec<Vec<i32>>) -> i32 {
        let len = a.len();
        let mut dp = a;

        for i in 1..len {
            for j in 0..len {
                dp[i][j] += *dp[i - 1][(j.max(1) - 1)..(j + 2).min(len)]
                    .iter()
                    .min()
                    .unwrap();
            }
        }

        *dp.last().unwrap().iter().min().unwrap()
    }
}
```
