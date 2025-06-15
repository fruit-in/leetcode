# 2536. Increment Submatrices by One
You are given a positive integer `n`, indicating that we initially have an `n x n` **0-indexed** integer matrix `mat` filled with zeroes.

You are also given a 2D integer array `query`. For each <code>query[i] = [row1<sub>i</sub>, col1<sub>i</sub>, row2<sub>i</sub>, col2<sub>i</sub>]</code>, you should do the following operation:
* Add `1` to **every element** in the submatrix with the **top left** corner <code>(row1<sub>i</sub>, col1<sub>i</sub>)</code> and the **bottom right** corner <code>(row2<sub>i</sub>, col2<sub>i</sub>)</code>. That is, add `1` to `mat[x][y]` for all <code>row1<sub>i</sub> <= x <= row2<sub>i</sub></code> and <code>col1<sub>i</sub> <= y <= col2<sub>i</sub></code>.

Return *the matrix* `mat` *after performing every query*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/11/24/p2example11.png)
<pre>
<strong>Input:</strong> n = 3, queries = [[1,1,2,2],[0,0,1,1]]
<strong>Output:</strong> [[1,1,0],[1,2,1],[0,1,1]]
<strong>Explanation:</strong> The diagram above shows the initial matrix, the matrix after the first query, and the matrix after the second query.
- In the first query, we add 1 to every element in the submatrix with the top left corner (1, 1) and bottom right corner (2, 2).
- In the second query, we add 1 to every element in the submatrix with the top left corner (0, 0) and bottom right corner (1, 1).
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/11/24/p2example22.png)
<pre>
<strong>Input:</strong> n = 2, queries = [[0,0,1,1]]
<strong>Output:</strong> [[1,1],[1,1]]
<strong>Explanation:</strong> The diagram above shows the initial matrix and the matrix after the first query.
- In the first query we add 1 to every element in the matrix.
</pre>

#### Constraints:
* `1 <= n <= 500`
* <code>1 <= queries.length <= 10<sup>4</sup></code>
* <code>0 <= row1<sub>i</sub> <= row2<sub>i</sub> < n</code>
* <code>0 <= col1<sub>i</sub> <= col2<sub>i</sub> < n</code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut mat = vec![vec![0; n]; n];

        for query in &queries {
            let (row1, col1, row2, col2) = (
                query[0] as usize,
                query[1] as usize,
                query[2] as usize,
                query[3] as usize,
            );

            mat[row2][col2] += 1;
            if row1 > 0 {
                mat[row1 - 1][col2] -= 1;
            }
            if col1 > 0 {
                mat[row2][col1 - 1] -= 1;
            }
            if row1 > 0 && col1 > 0 {
                mat[row1 - 1][col1 - 1] += 1;
            }
        }

        for row in (0..n).rev() {
            for col in (0..n).rev() {
                if row > 0 {
                    mat[row - 1][col] += mat[row][col];
                }
                if col > 0 {
                    mat[row][col - 1] += mat[row][col];
                }
                if row > 0 && col > 0 {
                    mat[row - 1][col - 1] -= mat[row][col];
                }
            }
        }

        mat
    }
}
```
