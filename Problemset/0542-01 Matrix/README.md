# 542. 01 Matrix
Given a matrix consists of 0 and 1, find the distance of the nearest 0 for each cell.

The distance between two adjacent cells is 1.

#### Example 1:
<pre>
<strong>Input:</strong>
[[0,0,0],
 [0,1,0],
 [0,0,0]]
<strong>Output:</strong>
[[0,0,0],
 [0,1,0],
 [0,0,0]]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
[[0,0,0],
 [0,1,0],
 [1,1,1]]
<strong>Output:</strong>
[[0,0,0],
 [0,1,0],
 [1,2,1]]
</pre>

#### Note:
1. The number of elements of the given matrix will not exceed 10,000.
2. There are at least one 0 in the given matrix.
3. The cells are adjacent in only four directions: up, down, left and right.

## Solutions (Rust)

### 1. Dynamic Programming
```Rust
impl Solution {
    pub fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ret = vec![vec![10000; matrix[0].len()]; matrix.len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                match matrix[i][j] {
                    0 => ret[i][j] = 0,
                    _ => {
                        if i > 0 {
                            ret[i][j] = ret[i - 1][j] + 1;
                        }
                        if j > 0 {
                            ret[i][j] = ret[i][j].min(ret[i][j - 1] + 1);
                        }
                    }
                }
            }
        }

        for i in (0..matrix.len()).rev() {
            for j in (0..matrix[0].len()).rev() {
                if matrix[i][j] == 1 {
                    if i + 1 < matrix.len() {
                        ret[i][j] = ret[i][j].min(ret[i + 1][j] + 1);
                    }
                    if j + 1 < matrix[0].len() {
                        ret[i][j] = ret[i][j].min(ret[i][j + 1] + 1);
                    }
                }
            }
        }

        ret
    }
}
```
