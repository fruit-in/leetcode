# 2146. K Highest Ranked Items Within a Price Range
You are given a **0-indexed** 2D integer array `grid` of size `m x n` that represents a map of the items in a shop. The integers in the grid represent the following:

* `0` represents a wall that you cannot pass through.
* `1` represents an empty cell that you can freely move to and from.
* All other positive integers represent the price of an item in that cell. You may also freely move to and from these item cells.

It takes `1` step to travel between adjacent grid cells.

You are also given integer arrays `pricing` and `start` where `pricing = [low, high]` and `start = [row, col]` indicates that you start at the position `(row, col)` and are interested only in items with a price in the range of `[low, high]` (**inclusive**). You are further given an integer `k`.

You are interested in the **positions** of the `k` **highest-ranked** items whose prices are **within** the given price range. The rank is determined by the **first** of these criteria that is different:

1. Distance, defined as the length of the shortest path from the `start` (**shorter** distance has a higher rank).
2. Price (**lower** price has a higher rank, but it must be **in the price range**).
3. The row number (**smaller** row number has a higher rank).
4. The column number (**smaller** column number has a higher rank).

Return *the* `k` *highest-ranked items within the price range **sorted** by their rank (highest to lowest)*. If there are fewer than `k` reachable items within the price range, return ***all** of them*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/12/16/example1drawio.png)
<pre>
<strong>Input:</strong> grid = [[1,2,0,1],[1,3,0,1],[0,2,5,1]], pricing = [2,5], start = [0,0], k = 3
<strong>Output:</strong> [[0,1],[1,1],[2,1]]
<strong>Explanation:</strong> You start at (0,0).
With a price range of [2,5], we can take items from (0,1), (1,1), (2,1) and (2,2).
The ranks of these items are:
- (0,1) with distance 1
- (1,1) with distance 2
- (2,1) with distance 3
- (2,2) with distance 4
Thus, the 3 highest ranked items in the price range are (0,1), (1,1), and (2,1).
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/12/16/example2drawio1.png)
<pre>
<strong>Input:</strong> grid = [[1,2,0,1],[1,3,3,1],[0,2,5,1]], pricing = [2,3], start = [2,3], k = 2
<strong>Output:</strong> [[2,1],[1,2]]
<strong>Explanation:</strong> You start at (2,3).
With a price range of [2,3], we can take items from (0,1), (1,1), (1,2) and (2,1).
The ranks of these items are:
- (2,1) with distance 2, price 2
- (1,2) with distance 2, price 3
- (1,1) with distance 3
- (0,1) with distance 4
Thus, the 2 highest ranked items in the price range are (2,1) and (1,2).
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/12/30/example3.png)
<pre>
<strong>Input:</strong> grid = [[1,1,1],[0,0,1],[2,3,4]], pricing = [2,3], start = [0,0], k = 3
<strong>Output:</strong> [[2,1],[2,0]]
<strong>Explanation:</strong> You start at (0,0).
With a price range of [2,3], we can take items from (2,0) and (2,1).
The ranks of these items are:
- (2,1) with distance 5
- (2,0) with distance 6
Thus, the 2 highest ranked items in the price range are (2,1) and (2,0).
Note that k = 3 but there are only 2 reachable items within the price range.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* <code>1 <= m, n <= 10<sup>5</sup></code>
* <code>1 <= m * n <= 10<sup>5</sup></code>
* <code>0 <= grid[i][j] <= 10<sup>5</sup></code>
* `pricing.length == 2`
* <code>2 <= low <= high <= 10<sup>5</sup></code>
* `start.length == 2`
* `0 <= row <= m - 1`
* `0 <= col <= n - 1`
* `grid[row][col] > 0`
* `1 <= k <= m * n`

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn highest_ranked_k_items(
        grid: Vec<Vec<i32>>,
        pricing: Vec<i32>,
        start: Vec<i32>,
        k: i32,
    ) -> Vec<Vec<i32>> {
        let mut grid = grid;
        let (low, high) = (pricing[0], pricing[1]);
        let (m, n) = (grid.len(), grid[0].len());
        let mut queue = VecDeque::from([(start[0] as usize, start[1] as usize, 0)]);
        let mut positions = vec![];

        while let Some((r, c, d)) = queue.pop_front() {
            if grid[r][c] == 0 {
                continue;
            }

            if grid[r][c] >= low && grid[r][c] <= high {
                positions.push((d, grid[r][c], r as i32, c as i32));
            }

            if r > 0 {
                queue.push_back((r - 1, c, d + 1));
            }
            if r < m - 1 {
                queue.push_back((r + 1, c, d + 1));
            }
            if c > 0 {
                queue.push_back((r, c - 1, d + 1));
            }
            if c < n - 1 {
                queue.push_back((r, c + 1, d + 1));
            }

            grid[r][c] = 0;
        }

        positions.sort_unstable();

        positions
            .iter()
            .take(k as usize)
            .map(|&(_, _, r, c)| vec![r, c])
            .collect()
    }
}
```
