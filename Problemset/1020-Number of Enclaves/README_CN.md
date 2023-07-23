# 1020. 飞地的数量
给出一个二维数组 `A`，每个单元格为 0（代表海）或 1（代表陆地）。

移动是指在陆地上从一个地方走到另一个地方（朝四个方向之一）或离开网格的边界。

返回网格中**无法**在任意次数的移动中离开网格边界的陆地单元格的数量。

#### 示例 1:
<pre>
<strong>输入:</strong> [[0,0,0,0],[1,0,1,0],[0,1,1,0],[0,0,0,0]]
<strong>输出:</strong> 3
<strong>解释:</strong>
有三个 1 被 0 包围。一个 1 没有被包围，因为它在边界上。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [[0,1,1,0],[0,0,1,0],[0,0,1,0],[0,0,0,0]]
<strong>输出:</strong> 0
<strong>解释:</strong>
所有 1 都在边界上或可以到达边界。
</pre>

#### 提示:
1. `1 <= A.length <= 500`
2. `1 <= A[i].length <= 500`
3. `0 <= A[i][j] <= 1`
4. 所有行的大小都相同

## 题解 (Rust)

### 1. 深度优先搜索
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn num_enclaves(a: Vec<Vec<i32>>) -> i32 {
        let m = a.len() as i32;
        let n = a[0].len() as i32;
        let mut seen = HashSet::new();
        let mut cells = Vec::new();
        let mut ret = 0;

        for i in 0..m {
            for j in 0..n {
                if a[i as usize][j as usize] == 1 {
                    if i == 0 || j == 0 || i == m - 1 || j == n - 1 {
                        seen.insert((i, j));
                        cells.push((i, j));
                    } else {
                        ret += 1;
                    }
                }
            }
        }

        while let Some((i, j)) = cells.pop() {
            for (x, y) in &[(-1, 0), (0, -1), (0, 1), (1, 0)] {
                let (i, j) = (i + x, j + y);
                if i > 0
                    && i < m - 1
                    && j > 0
                    && j < n - 1
                    && a[i as usize][j as usize] == 1
                    && !seen.contains(&(i, j))
                {
                    seen.insert((i, j));
                    cells.push((i, j));
                    ret -= 1;
                }
            }
        }

        ret
    }
}
```
