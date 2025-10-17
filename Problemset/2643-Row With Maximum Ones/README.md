# 2643. Row With Maximum Ones
Given a `m x n` binary matrix `mat`, find the **0-indexed** position of the row that contains the **maximum** count of **ones**, and the number of ones in that row.

In case there are multiple rows that have the maximum count of ones, the row with the **smallest row number** should be selected.

Return *an array containing the index of the row, and the number of ones in it*.

#### Example 1:
<pre>
<strong>Input:</strong> mat = [[0,1],[1,0]]
<strong>Output:</strong> [0,1]
<strong>Explanation:</strong> Both rows have the same number of 1's. So we return the index of the smaller row, 0, and the maximum count of ones (1). So, the answer is [0,1].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> mat = [[0,0,0],[0,1,1]]
<strong>Output:</strong> [1,2]
<strong>Explanation:</strong> The row indexed 1 has the maximum count of ones (2). So we return its index, 1, and the count. So, the answer is [1,2].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> mat = [[0,0],[1,1],[0,0]]
<strong>Output:</strong> [1,2]
<strong>Explanation:</strong> The row indexed 1 has the maximum count of ones (2). So the answer is [1,2].
</pre>

#### Constraints:
* `m == mat.length`
* `n == mat[i].length`
* `1 <= m, n <= 100`
* `mat[i][j]` is either `0` or `1`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn row_and_maximum_ones(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ret = vec![0, 0];

        for i in 0..mat.len() {
            let ones = mat[i].iter().sum::<i32>();

            if ones > ret[1] {
                ret = vec![i as i32, ones];
            }
        }

        ret
    }
}
```
