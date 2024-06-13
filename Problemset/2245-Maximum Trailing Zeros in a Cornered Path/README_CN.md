# 2245. 转角路径的乘积中最多能有几个尾随零
给你一个二维整数数组 `grid` ，大小为 `m x n`，其中每个单元格都含一个正整数。

**转角路径** 定义为：包含至多一个弯的一组相邻单元。具体而言，路径应该完全 **向水平方向** 或者 **向竖直方向** 移动过弯（如果存在弯），而不能访问之前访问过的单元格。在过弯之后，路径应当完全朝 **另一个** 方向行进：如果之前是向水平方向，那么就应该变为向竖直方向；反之亦然。当然，同样不能访问之前已经访问过的单元格。

一条路径的 **乘积** 定义为：路径上所有值的乘积。

请你从 `grid` 中找出一条乘积中尾随零数目最多的转角路径，并返回该路径中尾随零的数目。

注意：

* **水平** 移动是指向左或右移动。
* **竖直** 移动是指向上或下移动。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/03/23/ex1new2.jpg)
<pre>
<strong>输入:</strong> grid = [[23,17,15,3,20],[8,1,20,27,11],[9,4,6,2,21],[40,9,1,10,6],[22,7,4,5,3]]
<strong>输出:</strong> 3
<strong>解释:</strong> 左侧的图展示了一条有效的转角路径。
其乘积为 15 * 20 * 6 * 1 * 10 = 18000 ，共计 3 个尾随零。
可以证明在这条转角路径的乘积中尾随零数目最多。

中间的图不是一条有效的转角路径，因为它有不止一个弯。
右侧的图也不是一条有效的转角路径，因为它需要重复访问已经访问过的单元格。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/03/25/ex2.jpg)
<pre>
<strong>输入:</strong> grid = [[4,3,2],[7,6,1],[8,8,8]]
<strong>输出:</strong> 0
<strong>解释:</strong> 网格如上图所示。
不存在乘积含尾随零的转角路径。
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* <code>1 <= m, n <= 10<sup>5</sup></code>
* <code>1 <= m * n <= 10<sup>5</sup></code>
* `1 <= grid[i][j] <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_trailing_zeros(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut count25 = vec![vec![(0, 0); n]; m];
        let mut psud = count25.clone();
        let mut pslr = count25.clone();
        let mut psdu = count25.clone();
        let mut psrl = count25.clone();
        let mut ret = 0;

        for i in 0..m {
            for j in 0..n {
                let mut x = grid[i][j];

                while x % 2 == 0 {
                    count25[i][j].0 += 1;
                    x /= 2;
                }
                while x % 5 == 0 {
                    count25[i][j].1 += 1;
                    x /= 5;
                }

                psud[i][j] = count25[i][j];
                if i > 0 {
                    psud[i][j].0 += psud[i - 1][j].0;
                    psud[i][j].1 += psud[i - 1][j].1;
                }
                pslr[i][j] = count25[i][j];
                if j > 0 {
                    pslr[i][j].0 += pslr[i][j - 1].0;
                    pslr[i][j].1 += pslr[i][j - 1].1;
                }
            }
        }

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                psdu[i][j] = count25[i][j];
                if i < m - 1 {
                    psdu[i][j].0 += psdu[i + 1][j].0;
                    psdu[i][j].1 += psdu[i + 1][j].1;
                }
                psrl[i][j] = count25[i][j];
                if j < n - 1 {
                    psrl[i][j].0 += psrl[i][j + 1].0;
                    psrl[i][j].1 += psrl[i][j + 1].1;
                }

                ret = ret
                    .max(
                        (psud[i][j].0 + pslr[i][j].0 - count25[i][j].0)
                            .min(psud[i][j].1 + pslr[i][j].1 - count25[i][j].1),
                    )
                    .max(
                        (psud[i][j].0 + psrl[i][j].0 - count25[i][j].0)
                            .min(psud[i][j].1 + psrl[i][j].1 - count25[i][j].1),
                    )
                    .max(
                        (psdu[i][j].0 + pslr[i][j].0 - count25[i][j].0)
                            .min(psdu[i][j].1 + pslr[i][j].1 - count25[i][j].1),
                    )
                    .max(
                        (psdu[i][j].0 + psrl[i][j].0 - count25[i][j].0)
                            .min(psdu[i][j].1 + psrl[i][j].1 - count25[i][j].1),
                    );
            }
        }

        ret
    }
}
```
