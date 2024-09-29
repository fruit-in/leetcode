# 1765. Map of Highest Peak
You are given an integer matrix `isWater` of size `m x n` that represents a map of **land** and **water** cells.

* If `isWater[i][j] == 0`, cell `(i, j)` is a **land** cell.
* If `isWater[i][j] == 1`, cell `(i, j)` is a **water** cell.

You must assign each cell a height in a way that follows these rules:

* The height of each cell must be non-negative.
* If the cell is a **water** cell, its height must be `0`.
* Any two adjacent cells must have an absolute height difference of **at most** `1`. A cell is adjacent to another cell if the former is directly north, east, south, or west of the latter (i.e., their sides are touching).

Find an assignment of heights such that the maximum height in the matrix is **maximized**.

Return *an integer matrix* `height` *of size* `m x n` *where* `height[i][j]` *is cell* `(i, j)`*'s height. If there are multiple solutions, return **any** of them*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-82045-am.png)
<pre>
<strong>Input:</strong> isWater = [[0,1],[0,0]]
<strong>Output:</strong> [[1,0],[2,1]]
<strong>Explanation:</strong> The image shows the assigned heights of each cell.
The blue cell is the water cell, and the green cells are the land cells.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/01/10/screenshot-2021-01-11-at-82050-am.png)
<pre>
<strong>Input:</strong> isWater = [[0,0,1],[1,0,0],[0,0,0]]
<strong>Output:</strong> [[1,1,0],[0,1,1],[1,2,2]]
<strong>Explanation:</strong> A height of 2 is the maximum possible height of any assignment.
Any height assignment that has a maximum height of 2 while still meeting the rules will also be accepted.
</pre>

#### Constraints:
* `m == isWater.length`
* `n == isWater[i].length`
* `1 <= m, n <= 1000`
* `isWater[i][j]` is `0` or `1`.
* There is at least **one** water cell.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut is_water = is_water;
        let m = is_water.len();
        let n = is_water[0].len();
        let mut deque = VecDeque::new();
        let mut ret = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                if is_water[i][j] == 1 {
                    deque.push_back((i, j));
                }
            }
        }

        while let Some((i, j)) = deque.pop_front() {
            if i > 0 && is_water[i - 1][j] == 0 {
                is_water[i - 1][j] = 1;
                deque.push_back((i - 1, j));
                ret[i - 1][j] = ret[i][j] + 1;
            }
            if i + 1 < m && is_water[i + 1][j] == 0 {
                is_water[i + 1][j] = 1;
                deque.push_back((i + 1, j));
                ret[i + 1][j] = ret[i][j] + 1;
            }
            if j > 0 && is_water[i][j - 1] == 0 {
                is_water[i][j - 1] = 1;
                deque.push_back((i, j - 1));
                ret[i][j - 1] = ret[i][j] + 1;
            }
            if j + 1 < n && is_water[i][j + 1] == 0 {
                is_water[i][j + 1] = 1;
                deque.push_back((i, j + 1));
                ret[i][j + 1] = ret[i][j] + 1;
            }
        }

        ret
    }
}
```
