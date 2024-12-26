# 2257. Count Unguarded Cells in the Grid
You are given two integers `m` and `n` representing a **0-indexed** `m x n` grid. You are also given two 2D integer arrays `guards` and `walls` where <code>guards[i] = [row<sub>i</sub>, col<sub>i</sub>]</code> and <code>walls[j] = [row<sub>j</sub>, col<sub>j</sub>]</code> represent the positions of the <code>i<sup>th</sup></code> guard and <code>j<sup>th</sup></code> wall respectively.

A guard can see **every** cell in the four cardinal directions (north, east, south, or west) starting from their position unless **obstructed** by a wall or another **guard**. A cell is guarded if there is **at least** one guard that can see it.

Return *the number of unoccupied cells that are **not guarded***.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/03/10/example1drawio2.png)
<pre>
<strong>Input:</strong> m = 4, n = 6, guards = [[0,0],[1,1],[2,3]], walls = [[0,1],[2,2],[1,4]]
<strong>Output:</strong> 7
<strong>Explanation:</strong> The guarded and unguarded cells are shown in red and green respectively in the above diagram.
There are a total of 7 unguarded cells, so we return 7.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/03/10/example2drawio.png)
<pre>
<strong>Input:</strong> m = 3, n = 3, guards = [[1,1]], walls = [[0,1],[1,0],[2,1],[1,2]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The unguarded cells are shown in green in the above diagram.
There are a total of 4 unguarded cells, so we return 4.
</pre>

#### Constraints:
* <code>1 <= m, n <= 10<sup>5</sup></code>
* <code>2 <= m * n <= 10<sup>5</sup></code>
* <code>1 <= guards.length, walls.length <= 5 * 10<sup>4</sup></code>
* `2 <= guards.length + walls.length <= m * n`
* `guards[i].length == walls[j].length == 2`
* <code>0 <= row<sub>i</sub>, row<sub>j</sub> < m</code>
* <code>0 <= col<sub>i</sub>, col<sub>j</sub> < n</code>
* All the positions in `guards` and `walls` are **unique**.

## Solutions (Rust)

### 1. Solution
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
