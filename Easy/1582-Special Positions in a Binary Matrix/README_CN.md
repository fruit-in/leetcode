# 1582. 二进制矩阵中的特殊位置
给你一个大小为 `rows x cols` 的矩阵 `mat`，其中 `mat[i][j]` 是 `0` 或 `1`，请返回 **矩阵 *`mat`* 中特殊位置的数目** 。

**特殊位置** 定义：如果 `mat[i][j] == 1` 并且第 `i` 行和第 `j` 列中的所有其他元素均为 `0`（行和列的下标均 **从 0 开始** ），则位置 `(i, j)` 被称为特殊位置。

#### 示例 1:
<pre>
<b>输入:</b> mat = [[1,0,0],
            [0,0,<b>1</b>],
            [1,0,0]]
<b>输出:</b> 1
<b>解释:</b> (1,2) 是一个特殊位置，因为 mat[1][2] == 1 且所处的行和列上所有其他元素都是 0
</pre>

#### 示例 2:
<pre>
<b>输入:</b> mat = [[<b>1</b>,0,0],
            [0,<b>1</b>,0],
            [0,0,<b>1</b>]]
<b>输出:</b> 3
<b>解释:</b> (0,0), (1,1) 和 (2,2) 都是特殊位置
</pre>

#### 示例 3:
<pre>
<b>输入:</b> mat = [[0,0,0,<b>1</b>],
            [<b>1</b>,0,0,0],
            [0,1,1,0],
            [0,0,0,0]]
<b>输出:</b> 2
</pre>

#### 示例 4:
<pre>
<b>输入:</b> mat = [[0,0,0,0,0],
            [<b>1</b>,0,0,0,0],
            [0,<b>1</b>,0,0,0],
            [0,0,<b>1</b>,0,0],
            [0,0,0,1,1]]
<b>输出:</b> 3
</pre>

#### 提示:
* `rows == mat.length`
* `cols == mat[i].length`
* `1 <= rows, cols <= 100`
* `mat[i][j]` 是 `0` 或 `1`.

## 题解 (Rust)

### 1. 计数
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
