# 463. Island Perimeter
You are given a map in form of a two-dimensional integer grid where 1 represents land and 0 represents water.

Grid cells are connected horizontally/vertically (not diagonally). The grid is completely surrounded by water, and there is exactly one island (i.e., one or more connected land cells).

The island doesn't have "lakes" (water inside that isn't connected to the water around the island). One cell is a square with side length 1. The grid is rectangular, width and height don't exceed 100. Determine the perimeter of the island.

#### Example:
<pre>
<strong>Input:</strong>
[[0,1,0,0],
 [1,1,1,0],
 [0,1,0,0],
 [1,1,0,0]]
<strong>Output:</strong> 16
<strong>Explanation:</strong> The perimeter is the 16 yellow stripes in the image below:
<img src="https://assets.leetcode.com/uploads/2018/10/12/island.png">
</pre>

## Solutions (Rust)

### 1. Solution
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
