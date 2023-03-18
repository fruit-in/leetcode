# 1631. Path With Minimum Effort
You are a hiker preparing for an upcoming hike. You are given `heights`, a 2D array of size `rows x columns`, where `heights[row][col]` represents the height of cell `(row, col)`. You are situated in the top-left cell, `(0, 0)`, and you hope to travel to the bottom-right cell, `(rows-1, columns-1)` (i.e., **0-indexed**). You can move **up**, **down**, **left**, or **right**, and you wish to find a route that requires the minimum **effort**.

A route's **effort** is the **maximum absolute difference** in heights between two consecutive cells of the route.

Return *the minimum **effort** required to travel from the top-left cell to the bottom-right cell*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/10/04/ex1.png)
<pre>
<strong>Input:</strong> heights = [[1,2,2],[3,8,2],[5,3,5]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The route of [1,3,5,3,5] has a maximum absolute difference of 2 in consecutive cells.
This is better than the route of [1,2,2,2,5], where the maximum absolute difference is 3.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/10/04/ex2.png)
<pre>
<strong>Input:</strong> heights = [[1,2,3],[3,8,4],[5,3,5]]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The route of [1,2,3,4,5] has a maximum absolute difference of 1 in consecutive cells, which is better than route [1,3,5,3,5].
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2020/10/04/ex3.png)
<pre>
<strong>Input:</strong> heights = [[1,2,1,1,1],[1,2,1,2,1],[1,2,1,2,1],[1,2,1,2,1],[1,1,1,2,1]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> This route does not require any effort.
</pre>

#### Constraints:
* `rows == heights.length`
* `columns == heights[i].length`
* `1 <= rows, columns <= 100`
* <code>1 <= heights[i][j] <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let mut min_efforts = vec![vec![i32::MAX; heights[0].len()]; heights.len()];
        let mut cells = vec![(0, 0)];
        min_efforts[0][0] = 0;

        while let Some((i, j)) = cells.pop() {
            if i > 0 {
                let effort = (heights[i][j] - heights[i - 1][j])
                    .abs()
                    .max(min_efforts[i][j]);

                if effort < min_efforts[i - 1][j] {
                    min_efforts[i - 1][j] = effort;
                    cells.push((i - 1, j));
                }
            }
            if i < heights.len() - 1 {
                let effort = (heights[i][j] - heights[i + 1][j])
                    .abs()
                    .max(min_efforts[i][j]);

                if effort < min_efforts[i + 1][j] {
                    min_efforts[i + 1][j] = effort;
                    cells.push((i + 1, j));
                }
            }
            if j > 0 {
                let effort = (heights[i][j] - heights[i][j - 1])
                    .abs()
                    .max(min_efforts[i][j]);

                if effort < min_efforts[i][j - 1] {
                    min_efforts[i][j - 1] = effort;
                    cells.push((i, j - 1));
                }
            }
            if j < heights[0].len() - 1 {
                let effort = (heights[i][j] - heights[i][j + 1])
                    .abs()
                    .max(min_efforts[i][j]);

                if effort < min_efforts[i][j + 1] {
                    min_efforts[i][j + 1] = effort;
                    cells.push((i, j + 1));
                }
            }
        }

        *min_efforts.last().unwrap().last().unwrap()
    }
}
```
