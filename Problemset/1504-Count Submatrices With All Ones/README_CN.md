# 1504. 统计全 1 子矩形
给你一个 `m x n` 的二进制矩阵 `mat` ，请你返回有多少个 **子矩形** 的元素全部都是 1 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/10/27/ones1-grid.jpg)
<pre>
<strong>输入:</strong> mat = [[1,0,1],[1,1,0],[1,1,0]]
<strong>输出:</strong> 13
<strong>解释:</strong>
有 6 个 1x1 的矩形。
有 2 个 1x2 的矩形。
有 3 个 2x1 的矩形。
有 1 个 2x2 的矩形。
有 1 个 3x1 的矩形。
矩形数目总共 = 6 + 2 + 3 + 1 + 1 = 13 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/10/27/ones2-grid.jpg)
<pre>
<strong>输入:</strong> mat = [[0,1,1,0],[0,1,1,1],[1,1,1,0]]
<strong>输出:</strong> 24
<strong>解释:</strong>
有 8 个 1x1 的子矩形。
有 5 个 1x2 的子矩形。
有 2 个 1x3 的子矩形。
有 4 个 2x1 的子矩形。
有 2 个 2x2 的子矩形。
有 2 个 3x1 的子矩形。
有 1 个 3x2 的子矩形。
矩形数目总共 = 8 + 5 + 2 + 4 + 2 + 2 + 1 = 24 。
</pre>

#### 提示:
* `1 <= m, n <= 150`
* `mat[i][j]` 仅包含 `0` 或 `1`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut mat = mat;
        let mut ret = 0;

        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1 {
                    let mut min_count = i32::MAX;

                    if j > 0 {
                        mat[i][j] += mat[i][j - 1];
                    }

                    for k in (0..=i).rev() {
                        if mat[k][j] == 0 {
                            break;
                        }

                        min_count = min_count.min(mat[k][j]);
                        ret += min_count;
                    }
                }
            }
        }

        ret
    }
}
```
