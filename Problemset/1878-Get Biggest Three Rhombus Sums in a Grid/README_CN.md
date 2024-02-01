# 1878. 矩阵中最大的三个菱形和
给你一个 `m x n` 的整数矩阵 `grid` 。

**菱形和** 指的是 `grid` 中一个正菱形 **边界** 上的元素之和。本题中的菱形必须为正方形旋转45度，且四个角都在一个格子当中。下图是四个可行的菱形，每个菱形和应该包含的格子都用了相应颜色标注在图中。

![](https://assets.leetcode.com/uploads/2021/04/23/pc73-q4-desc-2.png)

注意，菱形可以是一个面积为 0 的区域，如上图中右下角的紫色菱形所示。

请你按照 **降序** 返回 `grid` 中三个最大的 **互不相同的菱形和** 。如果不同的和少于三个，则将它们全部返回。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/04/23/pc73-q4-ex1.png)
<pre>
<strong>输入:</strong> grid = [[3,4,5,1,3],[3,3,4,2,3],[20,30,200,40,10],[1,5,5,4,1],[4,3,2,2,5]]
<strong>输出:</strong> [228,216,211]
<strong>解释:</strong> 最大的三个菱形和如上图所示。
- 蓝色：20 + 3 + 200 + 5 = 228
- 红色：200 + 2 + 10 + 4 = 216
- 绿色：5 + 200 + 4 + 2 = 211
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/04/23/pc73-q4-ex2.png)
<pre>
<strong>输入:</strong> grid = [[1,2,3],[4,5,6],[7,8,9]]
<strong>输出:</strong> [20,9,8]
<strong>解释:</strong> 最大的三个菱形和如上图所示。
- 蓝色：4 + 2 + 6 + 8 = 20
- 红色：9 （右下角红色的面积为 0 的菱形）
- 绿色：8 （下方中央面积为 0 的菱形）
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> grid = [[7,7,7]]
<strong>输出:</strong> [7]
<strong>解释:</strong> 所有三个可能的菱形和都相同，所以返回 [7] 。
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 50`
* <code>1 <= grid[i][j] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BTreeSet;

impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();
        let mut tree_set = BTreeSet::new();
        let mut ret = vec![];

        for i in 0..m {
            for j in 0..n {
                tree_set.insert(grid[i][j]);

                for k in 1..=i.min(j).min(m - 1 - i).min(n - 1 - j) {
                    let mut rhombus_sum = 0;

                    for a in 0..k {
                        rhombus_sum += grid[i - a][j - k + a];
                        rhombus_sum += grid[i - k + a][j + a];
                        rhombus_sum += grid[i + a][j + k - a];
                        rhombus_sum += grid[i + k - a][j - a];
                    }

                    tree_set.insert(rhombus_sum);
                }
            }
        }

        while ret.len() < 3 && !tree_set.is_empty() {
            ret.push(tree_set.pop_last().unwrap());
        }

        ret
    }
}
```
