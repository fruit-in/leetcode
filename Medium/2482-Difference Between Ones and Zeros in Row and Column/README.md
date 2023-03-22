# 2482. Difference Between Ones and Zeros in Row and Column
You are given a **0-indexed** `m x n` binary matrix `grid`.

A **0-indexed** `m x n` difference matrix `diff` is created with the following procedure:

* Let the number of ones in the <code>i<sup>th</sup></code> row be <code>onesRow<sub>i</sub></code>.
* Let the number of ones in the <code>j<sup>th</sup></code> column be <code>onesCol<sub>j</sub></code>.
* Let the number of zeros in the <code>i<sup>th</sup></code> row be <code>zerosRow<sub>i</sub></code>.
* Let the number of zeros in the <code>j<sup>th</sup></code> column be <code>zerosCol<sub>j</sub></code>.
* <code>diff[i][j] = onesRow<sub>i</sub> + onesCol<sub>j</sub> - zerosRow<sub>i</sub> - zerosCol<sub>j</sub></code>

Return *the difference matrix* `diff`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/11/06/image-20221106171729-5.png)
<pre>
<strong>Input:</strong> grid = [[0,1,1],[1,0,1],[0,0,1]]
<strong>Output:</strong> [[0,0,4],[0,0,4],[-2,-2,2]]
<strong>Explanation:</strong>
- diff[0][0] = onesRow<sub>0</sub> + onesCol<sub>0</sub> - zerosRow<sub>0</sub> - zerosCol<sub>0</sub> = 2 + 1 - 1 - 2 = 0
- diff[0][1] = onesRow<sub>0</sub> + onesCol<sub>1</sub> - zerosRow<sub>0</sub> - zerosCol<sub>1</sub> = 2 + 1 - 1 - 2 = 0
- diff[0][2] = onesRow<sub>0</sub> + onesCol<sub>2</sub> - zerosRow<sub>0</sub> - zerosCol<sub>2</sub> = 2 + 3 - 1 - 0 = 4
- diff[1][0] = onesRow<sub>1</sub> + onesCol<sub>0</sub> - zerosRow<sub>1</sub> - zerosCol<sub>0</sub> = 2 + 1 - 1 - 2 = 0
- diff[1][1] = onesRow<sub>1</sub> + onesCol<sub>1</sub> - zerosRow<sub>1</sub> - zerosCol<sub>1</sub> = 2 + 1 - 1 - 2 = 0
- diff[1][2] = onesRow<sub>1</sub> + onesCol<sub>2</sub> - zerosRow<sub>1</sub> - zerosCol<sub>2</sub> = 2 + 3 - 1 - 0 = 4
- diff[2][0] = onesRow<sub>2</sub> + onesCol<sub>0</sub> - zerosRow<sub>2</sub> - zerosCol<sub>0</sub> = 1 + 1 - 2 - 2 = -2
- diff[2][1] = onesRow<sub>2</sub> + onesCol<sub>1</sub> - zerosRow<sub>2</sub> - zerosCol<sub>1</sub> = 1 + 1 - 2 - 2 = -2
- diff[2][2] = onesRow<sub>2</sub> + onesCol<sub>2</sub> - zerosRow<sub>2</sub> - zerosCol<sub>2</sub> = 1 + 3 - 2 - 0 = 2
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/11/06/image-20221106171747-6.png)
<pre>
<strong>Input:</strong> grid = [[1,1,1],[1,1,1]]
<strong>Output:</strong> [[5,5,5],[5,5,5]]
<strong>Explanation:</strong>
- diff[0][0] = onesRow<sub>0</sub> + onesCol<sub>0</sub> - zerosRow<sub>0</sub> - zerosCol<sub>0</sub> = 3 + 2 - 0 - 0 = 5
- diff[0][1] = onesRow<sub>0</sub> + onesCol<sub>1</sub> - zerosRow<sub>0</sub> - zerosCol<sub>1</sub> = 3 + 2 - 0 - 0 = 5
- diff[0][2] = onesRow<sub>0</sub> + onesCol<sub>2</sub> - zerosRow<sub>0</sub> - zerosCol<sub>2</sub> = 3 + 2 - 0 - 0 = 5
- diff[1][0] = onesRow<sub>1</sub> + onesCol<sub>0</sub> - zerosRow<sub>1</sub> - zerosCol<sub>0</sub> = 3 + 2 - 0 - 0 = 5
- diff[1][1] = onesRow<sub>1</sub> + onesCol<sub>1</sub> - zerosRow<sub>1</sub> - zerosCol<sub>1</sub> = 3 + 2 - 0 - 0 = 5
- diff[1][2] = onesRow<sub>1</sub> + onesCol<sub>2</sub> - zerosRow<sub>1</sub> - zerosCol<sub>2</sub> = 3 + 2 - 0 - 0 = 5
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* <code>1 <= m, n <= 10<sup>5</sup></code>
* <code>1 <= m * n <= 10<sup>5</sup></code>
* `grid[i][j]` is either `0` or `1`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut ones_row = vec![0; m];
        let mut ones_col = vec![0; n];
        let mut diff = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                ones_row[i] += grid[i][j];
                ones_col[j] += grid[i][j];
            }
        }

        for i in 0..m {
            for j in 0..n {
                diff[i][j] = 2 * ones_row[i] + 2 * ones_col[j] - (m + n) as i32;
            }
        }

        diff
    }
}
```
