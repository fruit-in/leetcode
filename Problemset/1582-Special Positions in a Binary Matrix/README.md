# 1582. Special Positions in a Binary Matrix
Given a `rows x cols` matrix `mat`, where `mat[i][j]` is either `0` or `1`, return *the number of special positions in `mat`*.

A position `(i,j)` is called **special** if `mat[i][j] == 1` and all other elements in row `i` and column `j` are `0` (rows and columns are **0-indexed**).

#### Example 1:
<pre>
<b>Input:</b> mat = [[1,0,0],
              [0,0,<b>1</b>],
              [1,0,0]]
<b>Output:</b> 1
<b>Explanation:</b> (1,2) is a special position because mat[1][2] == 1 and all other elements in row 1 and column 2 are 0.
</pre>

#### Example 2:
<pre>
<b>Input:</b> mat = [[<b>1</b>,0,0],
              [0,<b>1</b>,0],
              [0,0,<b>1</b>]]
<b>Output:</b> 3
<b>Explanation:</b> (0,0), (1,1) and (2,2) are special positions.
</pre>

#### Example 3:
<pre>
<b>Input:</b> mat = [[0,0,0,<b>1</b>],
              [<b>1</b>,0,0,0],
              [0,1,1,0],
              [0,0,0,0]]
<b>Output:</b> 2
</pre>

#### Example 4:
<pre>
<b>Input:</b> mat = [[0,0,0,0,0],
              [<b>1</b>,0,0,0,0],
              [0,<b>1</b>,0,0,0],
              [0,0,<b>1</b>,0,0],
              [0,0,0,1,1]]
<b>Output:</b> 3
</pre>

#### Constraints:
* `rows == mat.length`
* `cols == mat[i].length`
* `1 <= rows, cols <= 100`
* `mat[i][j]` is `0` or `1`.

## Solutions (Rust)

### 1. Count
```Rust
impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let rows = mat.len();
        let cols = mat[0].len();
        let mut row1 = vec![0; rows];
        let mut col1 = vec![0; cols];
        let mut ret = 0;

        for r in 0..rows {
            for c in 0..cols {
                if mat[r][c] == 1 {
                    row1[r] += 1;
                    col1[c] += 1;
                }
            }
        }

        for r in 0..rows {
            for c in 0..cols {
                if mat[r][c] == 1 && row1[r] + col1[c] == 2 {
                    ret += 1;
                }
            }
        }

        ret
    }
}
```
