# 417. Pacific Atlantic Water Flow
There is an `m x n` rectangular island that borders both the **Pacific Ocean** and **Atlantic Ocean**. The **Pacific Ocean** touches the island's left and top edges, and the **Atlantic Ocean** touches the island's right and bottom edges.

The island is partitioned into a grid of square cells. You are given an `m x n` integer matrix `heights` where `heights[r][c]` represents the **height above sea level** of the cell at coordinate `(r, c)`.

The island receives a lot of rain, and the rain water can flow to neighboring cells directly north, south, east, and west if the neighboring cell's height is **less than or equal to** the current cell's height. Water can flow from any cell adjacent to an ocean into the ocean.

Return *a **2D list** of grid coordinates* `result` *where* <code>result[i] = [r<sub>i</sub>, c<sub>i</sub>]</code> *denotes that rain water can flow from cell* <code>(r<sub>i</sub>, c<sub>i</sub>)</code> *to **both** the Pacific and Atlantic oceans*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/06/08/waterflow-grid.jpg)
<pre>
<strong>Input:</strong> heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
<strong>Output:</strong> [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
<strong>Explanation:</strong> The following cells can flow to the Pacific and Atlantic oceans, as shown below:
[0,4]: [0,4] -> Pacific Ocean
       [0,4] -> Atlantic Ocean
[1,3]: [1,3] -> [0,3] -> Pacific Ocean
       [1,3] -> [1,4] -> Atlantic Ocean
[1,4]: [1,4] -> [1,3] -> [0,3] -> Pacific Ocean
       [1,4] -> Atlantic Ocean
[2,2]: [2,2] -> [1,2] -> [0,2] -> Pacific Ocean
       [2,2] -> [2,3] -> [2,4] -> Atlantic Ocean
[3,0]: [3,0] -> Pacific Ocean
       [3,0] -> [4,0] -> Atlantic Ocean
[3,1]: [3,1] -> [3,0] -> Pacific Ocean
       [3,1] -> [4,1] -> Atlantic Ocean
[4,0]: [4,0] -> Pacific Ocean
       [4,0] -> Atlantic Ocean
Note that there are other possible paths for these cells to flow to the Pacific and Atlantic oceans.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> heights = [[1]]
<strong>Output:</strong> [[0,0]]
<strong>Explanation:</strong> The water can flow from the only cell to the Pacific and Atlantic oceans.
</pre>

#### Constraints:
* `m == heights.length`
* `n == heights[r].length`
* `1 <= m, n <= 200`
* <code>0 <= heights[r][c] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = heights.len();
        let n = heights[0].len();
        let mut cells = vec![];
        let mut pacific = HashSet::new();
        let mut atlantic = HashSet::new();

        for r in 0..m {
            cells.push((r, 0));
            cells.push((r, n - 1));
            pacific.insert((r, 0));
            atlantic.insert((r, n - 1));
        }
        for c in 0..n {
            cells.push((0, c));
            cells.push((m - 1, c));
            pacific.insert((0, c));
            atlantic.insert((m - 1, c));
        }

        while let Some((r0, c0)) = cells.pop() {
            for (x, y) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
                let r1 = ((r0 as i32 + x).max(0) as usize).min(m - 1);
                let c1 = ((c0 as i32 + y).max(0) as usize).min(n - 1);

                if heights[r0][c0] > heights[r1][c1] {
                    continue;
                }
                if pacific.contains(&(r0, c0)) && pacific.insert((r1, c1)) {
                    cells.push((r1, c1));
                }
                if atlantic.contains(&(r0, c0)) && atlantic.insert((r1, c1)) {
                    cells.push((r1, c1));
                }
            }
        }

        pacific
            .intersection(&atlantic)
            .map(|&(r, c)| vec![r as i32, c as i32])
            .collect()
    }
}
```
