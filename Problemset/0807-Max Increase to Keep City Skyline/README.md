# 807. Max Increase to Keep City Skyline
In a 2 dimensional array ```grid```, each value ```grid[i][j]``` represents the height of a building located there. We are allowed to increase the height of any number of buildings, by any amount (the amounts can be different for different buildings). Height 0 is considered to be a building as well.

At the end, the "skyline" when viewed from all four directions of the grid, i.e. top, bottom, left, and right, must be the same as the skyline of the original grid. A city's skyline is the outer contour of the rectangles formed by all the buildings when viewed from a distance. See the following example.

What is the maximum total sum that the height of the buildings can be increased?

<pre>
<strong>Example:</strong>
<strong>Input:</strong> grid = [[3,0,8,4],[2,4,5,7],[9,2,6,3],[0,3,1,0]]
<strong>Output:</strong> 35
<strong>Explanation:</strong>
The grid is:
[ [3, 0, 8, 4],
  [2, 4, 5, 7],
  [9, 2, 6, 3],
  [0, 3, 1, 0] ]

The skyline viewed from top or bottom is: [9, 4, 8, 7]
The skyline viewed from left or right is: [8, 7, 9, 3]

The grid after increasing the height of buildings without affecting skylines is:

gridNew = [ [8, 4, 8, 7],
            [7, 4, 7, 7],
            [9, 4, 8, 7],
            [3, 3, 3, 3] ]
</pre>

#### Notes:
* ```1 < grid.length = grid[0].length <= 50```.
* All heights ```grid[i][j]``` are in the range ```[0, 100]```.
* All buildings in ```grid[i][j]``` occupy the entire grid cell: that is, they are a ```1 x 1 x grid[i][j]``` rectangular prism.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let mut row_max = Vec::new();
        let mut col_max = Vec::new();
        let mut ret = 0;

        for i in 0..grid.len() {
            row_max.push(*grid[i].iter().max().unwrap());
            col_max.push(grid.iter().map(|v| v[i]).max().unwrap());
        }

        for i in 0..grid.len() {
            for j in 0..grid.len() {
                ret += row_max[i].min(col_max[j]) - grid[i][j];
            }
        }

        ret
    }
}
```
