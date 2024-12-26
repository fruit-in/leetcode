# 2257. 统计网格图中没有被保卫的格子数
给你两个整数 `m` 和 `n` 表示一个下标从 **0** 开始的 `m x n` 网格图。同时给你两个二维整数数组 `guards` 和 `walls` ，其中 <code>guards[i] = [row<sub>i</sub>, col<sub>i</sub>]</code> 且 <code>walls[j] = [row<sub>j</sub>, col<sub>j</sub>]</code> ，分别表示第 `i` 个警卫和第 `j` 座墙所在的位置。

一个警卫能看到 4 个坐标轴方向（即东、南、西、北）的 **所有** 格子，除非他们被一座墙或者另外一个警卫 **挡住** 了视线。如果一个格子能被 **至少** 一个警卫看到，那么我们说这个格子被 **保卫** 了。

请你返回空格子中，有多少个格子是 **没被保卫** 的。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/03/10/example1drawio2.png)
<pre>
<strong>输入:</strong> m = 4, n = 6, guards = [[0,0],[1,1],[2,3]], walls = [[0,1],[2,2],[1,4]]
<strong>输出:</strong> 7
<strong>解释:</strong> 上图中，被保卫和没有被保卫的格子分别用红色和绿色表示。
总共有 7 个没有被保卫的格子，所以我们返回 7 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/03/10/example2drawio.png)
<pre>
<strong>输入:</strong> m = 3, n = 3, guards = [[1,1]], walls = [[0,1],[1,0],[2,1],[1,2]]
<strong>输出:</strong> 4
<strong>解释:</strong> 上图中，没有被保卫的格子用绿色表示。
总共有 4 个没有被保卫的格子，所以我们返回 4 。
</pre>

#### 提示:
* <code>1 <= m, n <= 10<sup>5</sup></code>
* <code>2 <= m * n <= 10<sup>5</sup></code>
* <code>1 <= guards.length, walls.length <= 5 * 10<sup>4</sup></code>
* `2 <= guards.length + walls.length <= m * n`
* `guards[i].length == walls[j].length == 2`
* <code>0 <= row<sub>i</sub>, row<sub>j</sub> < m</code>
* <code>0 <= col<sub>i</sub>, col<sub>j</sub> < n</code>
* `guards` 和 `walls` 中所有位置 **互不相同** 。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut grid = vec![vec![0; n]; m];
        let mut ret = 0;

        for i in 0..guards.len() {
            grid[guards[i][0] as usize][guards[i][1] as usize] = 1;
        }
        for i in 0..walls.len() {
            grid[walls[i][0] as usize][walls[i][1] as usize] = 2;
        }

        for r in 0..m {
            let mut can_see = false;

            for c in 0..n {
                match grid[r][c] {
                    1 => can_see = true,
                    2 => can_see = false,
                    _ if can_see => grid[r][c] = 3,
                    _ => (),
                }
            }

            can_see = false;

            for c in (0..n).rev() {
                match grid[r][c] {
                    1 => can_see = true,
                    2 => can_see = false,
                    _ if can_see => grid[r][c] = 3,
                    _ => (),
                }
            }
        }

        for c in 0..n {
            let mut can_see = false;

            for r in 0..m {
                match grid[r][c] {
                    1 => can_see = true,
                    2 => can_see = false,
                    _ if can_see => grid[r][c] = 3,
                    _ => (),
                }
            }

            can_see = false;

            for r in (0..m).rev() {
                match grid[r][c] {
                    1 => can_see = true,
                    2 => can_see = false,
                    _ if can_see => grid[r][c] = 3,
                    _ => (),
                }

                if grid[r][c] == 0 {
                    ret += 1;
                }
            }
        }

        ret
    }
}
```
