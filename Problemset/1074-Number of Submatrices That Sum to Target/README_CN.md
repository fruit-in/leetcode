# 1074. 元素和为目标值的子矩阵数量
给出矩阵 `matrix` 和目标值 `target`，返回元素总和等于目标值的非空子矩阵的数量。

子矩阵 `x1, y1, x2, y2` 是满足 `x1 <= x <= x2` 且 `y1 <= y <= y2` 的所有单元 `matrix[x][y]` 的集合。

如果 `(x1, y1, x2, y2)` 和 `(x1', y1', x2', y2')` 两个子矩阵中部分坐标不同（如：`x1 != x1'`），那么这两个子矩阵也不同。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/09/02/mate1.jpg)
<pre>
<strong>输入:</strong> matrix = [[0,1,0],[1,1,1],[0,1,0]], target = 0
<strong>输出:</strong> 4
<strong>解释:</strong> 四个只含 0 的 1x1 子矩阵。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> matrix = [[1,-1],[-1,1]], target = 0
<strong>输出:</strong> 5
<strong>解释:</strong> 两个 1x2 子矩阵，加上两个 2x1 子矩阵，再加上一个 2x2 子矩阵。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> matrix = [[904]], target = 0
<strong>输出:</strong> 0
</pre>

#### 提示:
* `1 <= matrix.length <= 100`
* `1 <= matrix[0].length <= 100`
* `-1000 <= matrix[i] <= 1000`
* `-10^8 <= target <= 10^8`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut matrix = matrix;
        let mut count = HashMap::new();
        let mut ret = 0;

        for x1 in 0..m {
            for x2 in x1..m {
                count.insert((x1, x2, 0), 1);
            }
        }

        for x2 in 0..m {
            for y2 in 0..n {
                if x2 > 0 {
                    matrix[x2][y2] += matrix[x2 - 1][y2];
                }
                if y2 > 0 {
                    matrix[x2][y2] += matrix[x2][y2 - 1];
                }
                if x2 > 0 && y2 > 0 {
                    matrix[x2][y2] -= matrix[x2 - 1][y2 - 1];
                }

                for x1 in 0..=x2 {
                    let mut diff = matrix[x2][y2];
                    if x1 > 0 {
                        diff -= matrix[x1 - 1][y2];
                    }
                    ret += *count.get(&(x1, x2, diff - target)).unwrap_or(&0);
                    *count.entry((x1, x2, diff)).or_insert(0) += 1;
                }
            }
        }

        ret
    }
}
```
