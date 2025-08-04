# 1254. Number of Closed Islands
Given a 2D `grid` consists of `0s` (land) and `1s` (water).  An *island* is a maximal 4-directionally connected group of `0s` and a closed island is an island **totally** (all left, top, right, bottom) surrounded by `1s`.

Return the number of *closed islands*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/10/31/sample_3_1610.png)
<pre>
<strong>Input:</strong> grid = [[1,1,1,1,1,1,1,0],[1,0,0,0,0,1,1,0],[1,0,1,0,1,1,1,0],[1,0,0,0,0,1,0,1],[1,1,1,1,1,1,1,0]]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
Islands in gray are closed because they are completely surrounded by water (group of 1s).
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2019/10/31/sample_4_1610.png)
<pre>
<strong>Input:</strong> grid = [[0,0,1,0,0],[0,1,0,1,0],[0,1,1,1,0]]
<strong>Output:</strong> 1
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> grid = [[1,1,1,1,1,1,1],
               [1,0,0,0,0,0,1],
               [1,0,1,1,1,0,1],
               [1,0,1,0,1,0,1],
               [1,0,1,1,1,0,1],
               [1,0,0,0,0,0,1],
               [1,1,1,1,1,1,1]]
<strong>Output:</strong> 2
</pre>

#### Constraints:
* `1 <= grid.length, grid[0].length <= 100`
* `0 <= grid[i][j] <=1`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut stack = vec![];
        let mut ret = 0;

        for r in 0..m {
            if grid[r][0] == 0 {
                grid[r][0] = 1;
                stack.push((r, 0));
            }
            if grid[r][n - 1] == 0 {
                grid[r][n - 1] = 1;
                stack.push((r, n - 1));
            }
        }
        for c in 1..n - 1 {
            if grid[0][c] == 0 {
                grid[0][c] = 1;
                stack.push((0, c));
            }
            if grid[m - 1][c] == 0 {
                grid[m - 1][c] = 1;
                stack.push((m - 1, c));
            }
        }

        while let Some((r, c)) = stack.pop() {
            if r > 0 && grid[r - 1][c] == 0 {
                grid[r - 1][c] = 1;
                stack.push((r - 1, c));
            }
            if r < m - 1 && grid[r + 1][c] == 0 {
                grid[r + 1][c] = 1;
                stack.push((r + 1, c));
            }
            if c > 0 && grid[r][c - 1] == 0 {
                grid[r][c - 1] = 1;
                stack.push((r, c - 1));
            }
            if c < n - 1 && grid[r][c + 1] == 0 {
                grid[r][c + 1] = 1;
                stack.push((r, c + 1));
            }
        }

        for r in 1..m - 1 {
            for c in 1..n - 1 {
                if grid[r][c] == 1 {
                    continue;
                }

                grid[r][c] = 1;
                stack.push((r, c));
                while let Some((r, c)) = stack.pop() {
                    if r > 0 && grid[r - 1][c] == 0 {
                        grid[r - 1][c] = 1;
                        stack.push((r - 1, c));
                    }
                    if r < m - 1 && grid[r + 1][c] == 0 {
                        grid[r + 1][c] = 1;
                        stack.push((r + 1, c));
                    }
                    if c > 0 && grid[r][c - 1] == 0 {
                        grid[r][c - 1] = 1;
                        stack.push((r, c - 1));
                    }
                    if c < n - 1 && grid[r][c + 1] == 0 {
                        grid[r][c + 1] = 1;
                        stack.push((r, c + 1));
                    }
                }

                ret += 1;
            }
        }

        ret
    }
}
```
