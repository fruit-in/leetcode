# 2536. 子矩阵元素加 1
给你一个正整数 `n` ，表示最初有一个 `n x n` 、下标从 **0** 开始的整数矩阵 `mat` ，矩阵中填满了 0 。

另给你一个二维整数数组 `query` 。针对每个查询 <code>query[i] = [row1<sub>i</sub>, col1<sub>i</sub>, row2<sub>i</sub>, col2<sub>i</sub>]</code> ，请你执行下述操作：
* 找出 **左上角** 为 <code>(row1<sub>i</sub>, col1<sub>i</sub>)</code> 且 **右下角** 为 <code>(row2<sub>i</sub>, col2<sub>i</sub>)</code> 的子矩阵，将子矩阵中的 **每个元素** 加 `1` 。也就是给所有满足 <code>row1<sub>i</sub> <= x <= row2<sub>i</sub></code> 和 <code>col1<sub>i</sub> <= y <= col2<sub>i</sub></code> 的 `mat[x][y]` 加 `1` 。

返回执行完所有操作后得到的矩阵 `mat` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/11/24/p2example11.png)
<pre>
<strong>输入:</strong> n = 3, queries = [[1,1,2,2],[0,0,1,1]]
<strong>输出:</strong> [[1,1,0],[1,2,1],[0,1,1]]
<strong>解释:</strong> 上图所展示的分别是：初始矩阵、执行完第一个操作后的矩阵、执行完第二个操作后的矩阵。
- 第一个操作：将左上角为 (1, 1) 且右下角为 (2, 2) 的子矩阵中的每个元素加 1 。
- 第二个操作：将左上角为 (0, 0) 且右下角为 (1, 1) 的子矩阵中的每个元素加 1 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/11/24/p2example22.png)
<pre>
<strong>输入:</strong> n = 2, queries = [[0,0,1,1]]
<strong>输出:</strong> [[1,1],[1,1]]
<strong>解释:</strong> 上图所展示的分别是：初始矩阵、执行完第一个操作后的矩阵。
- 第一个操作：将矩阵中的每个元素加 1 。
</pre>

#### 提示:
* `1 <= n <= 500`
* <code>1 <= queries.length <= 10<sup>4</sup></code>
* <code>0 <= row1<sub>i</sub> <= row2<sub>i</sub> < n</code>
* <code>0 <= col1<sub>i</sub> <= col2<sub>i</sub> < n</code>

## 题解 (Rust)

### 1. 题解
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
