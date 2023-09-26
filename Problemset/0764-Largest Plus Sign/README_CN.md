# 764. 最大加号标志
在一个 `n x n` 的矩阵 `grid` 中，除了在数组 `mines` 中给出的元素为 `0`，其他每个元素都为 `1`。<code>mines[i] = [x<sub>i</sub>, y<sub>i</sub>]</code>表示 <code>grid[x<sub>i</sub>][y<sub>i</sub>] == 0</code>

返回  `grid` *中包含* `1` *的最大的 **轴对齐** 加号标志的阶数* 。如果未找到加号标志，则返回 `0` 。

一个 `k` 阶由 `1` 组成的 **“轴对称”加号标志** 具有中心网格 `grid[r][c] == 1` ，以及4个从中心向上、向下、向左、向右延伸，长度为 `k-1`，由 `1` 组成的臂。注意，只有加号标志的所有网格要求为 `1` ，别的网格可能为 `0` 也可能为 `1` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/06/13/plus1-grid.jpg)
<pre>
<strong>输入:</strong> n = 5, mines = [[4,2]]
<strong>输出:</strong> 2
<strong>解释:</strong> 在上面的网格中，最大加号标志的阶只能是2。一个标志已在图中标出。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 1, mines = [[0,0]]
<strong>输出:</strong> 0
<strong>解释:</strong> 没有加号标志，返回 0 。
</pre>

#### 提示:
* `1 <= n <= 500`
* `1 <= mines.length <= 5000`
* <code>0 <= x<sub>i</sub>, y<sub>i</sub> < n</code>
* 每一对 <code>(x<sub>i</sub>, y<sub>i</sub>)</code> 都 **不重复**

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut grid = vec![vec![[1; 5]; n]; n];
        let mut ret = 0;

        for mine in &mines {
            grid[mine[0] as usize][mine[1] as usize] = [0; 5];
        }

        for x in 0..n {
            for y in 1..n {
                if grid[x][y][0] == 1 {
                    grid[x][y][1] += grid[x][y - 1][1];
                }
                if grid[y][x][0] == 1 {
                    grid[y][x][2] += grid[y - 1][x][2];
                }
            }
            for y in (0..n - 1).rev() {
                if grid[x][y][0] == 1 {
                    grid[x][y][3] += grid[x][y + 1][3];
                }
                if grid[y][x][0] == 1 {
                    grid[y][x][4] += grid[y + 1][x][4];
                }
            }
        }

        for x in 0..n {
            for y in 0..n {
                if grid[x][y][0] == 1 {
                    ret = ret.max(*grid[x][y][1..].iter().min().unwrap());
                }
            }
        }

        ret
    }
}
```
