# 1727. 重新排列后的最大子矩阵
给你一个二进制矩阵 `matrix` ，它的大小为 `m x n` ，你可以将 `matrix` 中的 **列** 按任意顺序重新排列。

请你返回最优方案下将 `matrix` 重新排列后，全是 `1` 的子矩阵面积。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/12/29/screenshot-2020-12-30-at-40536-pm.png)
<pre>
<strong>输入:</strong> matrix = [[0,0,1],[1,1,1],[1,0,1]]
<strong>输出:</strong> 4
<strong>解释:</strong> 你可以按照上图方式重新排列矩阵的每一列。
最大的全 1 子矩阵是上图中加粗的部分，面积为 4 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/12/29/screenshot-2020-12-30-at-40852-pm.png)
<pre>
<strong>输入:</strong> matrix = [[1,0,1,0,1]]
<strong>输出:</strong> 3
<strong>解释:</strong> 你可以按照上图方式重新排列矩阵的每一列。
最大的全 1 子矩阵是上图中加粗的部分，面积为 3 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> matrix = [[1,1,0],[1,0,1]]
<strong>输出:</strong> 2
<strong>解释:</strong> 由于你只能整列整列重新排布，所以没有比面积为 2 更大的全 1 子矩形。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> matrix = [[0,0],[0,0]]
<strong>输出:</strong> 0
<strong>解释:</strong> 由于矩阵中没有 1 ，没有任何全 1 的子矩阵，所以面积为 0 。
</pre>

#### 提示:
* `m == matrix.length`
* `n == matrix[i].length`
* <code>1 <= m * n <= 10<sup>5</sup></code>
* `matrix[i][j]` 要么是 `0` ，要么是 `1` 。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut consecutive_ones = vec![0; n];
        let mut ret = 0;

        for r in 0..m {
            let mut heap = BinaryHeap::new();

            for c in 0..n {
                consecutive_ones[c] = (consecutive_ones[c] + 1) * matrix[r][c];
                if consecutive_ones[c] > 0 {
                    heap.push(consecutive_ones[c]);
                }
            }

            for length in 1..=heap.len() as i32 {
                ret = ret.max(length * heap.pop().unwrap());
            }
        }

        ret
    }
}
```
