# 1162. 地图分析
你现在手里有一份大小为 N x N 的 网格 `grid`，上面的每个 单元格 都用 `0` 和 `1` 标记好了。其中 `0` 代表海洋，`1` 代表陆地，请你找出一个海洋单元格，这个海洋单元格到离它最近的陆地单元格的距离是最大的。

我们这里说的距离是「曼哈顿距离」（ Manhattan Distance）：`(x0, y0)` 和 `(x1, y1)` 这两个单元格之间的距离是 `|x0 - x1| + |y0 - y1|` 。

如果网格上只有陆地或者海洋，请返回 `-1`。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/08/17/1336_ex1.jpeg)
<pre>
<strong>输入:</strong> grid = [[1,0,1],[0,0,0],[1,0,1]]
<strong>输出:</strong> 2
<strong>解释:</strong> 海洋单元格 (1, 1) 和所有陆地单元格之间的距离都达到最大，最大距离为 2。
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/08/17/1336_ex2.jpeg)
<pre>
<strong>输入:</strong> grid = [[1,0,0],[0,0,0],[0,0,0]]
<strong>输出:</strong> 4
<strong>解释:</strong> 海洋单元格 (2, 2) 和所有陆地单元格之间的距离都达到最大，最大距离为 4。
</pre>

#### 提示:
1. `1 <= grid.length == grid[0].length <= 100`
2. `grid[i][j]` 不是 `0` 就是 `1`

## 题解 (Rust)

### 1. 广度优先搜索
```Rust
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len() as i32;
        let mut ret = -1;

        for i in 0..n {
            for j in 0..n {
                if grid[i as usize][j as usize] == 1 {
                    continue;
                }

                let mut cells = vec![(i, j, 0)].into_iter().collect::<VecDeque<_>>();
                let mut seen = vec![(i, j)].into_iter().collect::<HashSet<_>>();
                while let Some((x, y, d)) = cells.pop_front() {
                    if grid[x as usize][y as usize] == 1 {
                        ret = ret.max(d);
                        break;
                    }
                    for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        let x = x + dx;
                        let y = y + dy;
                        if x >= 0 && x < n && y >= 0 && y < n && !seen.contains(&(x, y)) {
                            cells.push_back((x, y, d + 1));
                            seen.insert((x, y));
                        }
                    }
                }
            }
        }

        ret
    }
}
```
