# 1463. Cherry Pickup II
You are given a `rows x cols` matrix `grid` representing a field of cherries where `grid[i][j]` represents the number of cherries that you can collect from the `(i, j)` cell.

You have two robots that can collect cherries for you:

* **Robot #1** is located at the **top-left corner** `(0, 0)`, and
* **Robot #2** is located at the **top-right corner** `(0, cols - 1)`.

Return *the maximum number of cherries collection using both robots by following the rules below*:

* From a cell `(i, j)`, robots can move to cell `(i + 1, j - 1)`, `(i + 1, j)`, or `(i + 1, j + 1)`.
* When any robot passes through a cell, It picks up all cherries, and the cell becomes an empty cell.
* When both robots stay in the same cell, only one takes the cherries.
* Both robots cannot move outside of the grid at any moment.
* Both robots should reach the bottom row in `grid`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/04/29/sample_1_1802.png)
<pre>
<strong>Input:</strong> grid = [[3,1,1],[2,5,1],[1,5,5],[2,1,1]]
<strong>Output:</strong> 24
<strong>Explanation:</strong> Path of robot #1 and #2 are described in color green and blue respectively.
Cherries taken by Robot #1, (3 + 2 + 5 + 2) = 12.
Cherries taken by Robot #2, (1 + 5 + 5 + 1) = 12.
Total of cherries: 12 + 12 = 24.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/04/23/sample_2_1802.png)
<pre>
<strong>Input:</strong> grid = [[1,0,0,0,0,0,1],[2,0,0,0,0,3,0],[2,0,9,0,0,0,0],[0,3,0,5,4,0,0],[1,0,2,3,0,0,6]]
<strong>Output:</strong> 28
<strong>Explanation:</strong> Path of robot #1 and #2 are described in color green and blue respectively.
Cherries taken by Robot #1, (1 + 9 + 5 + 2) = 17.
Cherries taken by Robot #2, (1 + 3 + 4 + 3) = 11.
Total of cherries: 17 + 11 = 28.
</pre>

#### Constraints:
* `rows == grid.length`
* `cols == grid[i].length`
* `2 <= rows, cols <= 70`
* `0 <= grid[i][j] <= 100`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut curr_row = vec![vec![None; cols]; cols];
        curr_row[0][cols - 1] = Some(grid[0][0] + grid[0][cols - 1]);

        for i in 0..rows - 1 {
            let mut next_row = vec![vec![None; cols]; cols];

            for j0 in 0..cols {
                for j1 in 0..cols {
                    if let Some(x) = curr_row[j0][j1] {
                        for c0 in j0.saturating_sub(1)..cols.min(j0 + 2) {
                            for c1 in j1.saturating_sub(1)..cols.min(j1 + 2) {
                                next_row[c0][c1] = Some(next_row[c0][c1].unwrap_or(0).max(
                                    x + grid[i + 1][c0] + grid[i + 1][c1] * (c0 != c1) as i32,
                                ));
                            }
                        }
                    }
                }
            }

            curr_row = next_row;
        }

        curr_row
            .iter()
            .flatten()
            .map(|x| x.unwrap_or(0))
            .max()
            .unwrap()
    }
}
```
