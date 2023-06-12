# 2428. 沙漏的最大总和
给你一个大小为 `m x n` 的整数矩阵 `grid` 。

按以下形式将矩阵的一部分定义为一个 **沙漏** ：

![](https://assets.leetcode.com/uploads/2022/08/21/img.jpg)

返回沙漏中元素的 **最大** 总和。

**注意：**沙漏无法旋转且必须整个包含在矩阵中。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/08/21/1.jpg)
<pre>
<strong>输入:</strong> grid = [[6,2,1,3],[4,2,1,5],[9,2,8,7],[4,1,2,9]]
<strong>输出:</strong> 30
<strong>解释:</strong> 上图中的单元格表示元素总和最大的沙漏：6 + 2 + 1 + 2 + 9 + 2 + 8 = 30 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/08/21/2.jpg)
<pre>
<strong>输入:</strong> grid = [[1,2,3],[4,5,6],[7,8,9]]
<strong>输出:</strong> 35
<strong>解释:</strong> 上图中的单元格表示元素总和最大的沙漏：1 + 2 + 3 + 5 + 7 + 8 + 9 = 35 。
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `3 <= m, n <= 150`
* <code>0 <= grid[i][j] <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;

        for i in 1..grid.len() - 1 {
            for j in 1..grid[0].len() - 1 {
                ret = ret.max(
                    grid[i][j]
                        + grid[i - 1][j]
                        + grid[i + 1][j]
                        + grid[i - 1][j - 1]
                        + grid[i + 1][j - 1]
                        + grid[i - 1][j + 1]
                        + grid[i + 1][j + 1],
                );
            }
        }

        ret
    }
}
```
