# 463. 岛屿的周长
给定一个包含 0 和 1 的二维网格地图，其中 1 表示陆地 0 表示水域。

网格中的格子水平和垂直方向相连（对角线方向不相连）。整个网格被水完全包围，但其中恰好有一个岛屿（或者说，一个或多个表示陆地的格子相连组成的岛屿）。

岛屿中没有“湖”（“湖” 指水域在岛屿内部且不和岛屿周围的水相连）。格子是边长为 1 的正方形。网格为长方形，且宽度和高度均不超过 100 。计算这个岛屿的周长。

#### 示例:
<pre>
<strong>输入:</strong>
[[0,1,0,0],
 [1,1,1,0],
 [0,1,0,0],
 [1,1,0,0]]
<strong>输出:</strong> 16
<strong>解释:</strong> 它的周长是下面图片中的 16 个黄色的边：
<img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/island.png">
</pre>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut perimeter = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    perimeter += 2;
                    if i > 0 {
                        perimeter -= grid[i - 1][j];
                    }
                    if j > 0 {
                        perimeter -= grid[i][j - 1];
                    }
                }
            }
        }

        2 * perimeter
    }
}
```
