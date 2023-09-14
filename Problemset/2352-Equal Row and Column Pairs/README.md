# 2352. Equal Row and Column Pairs
Given a **0-indexed** `n x n` integer matrix `grid`, *return the number of pairs* <code>(r<sub>i</sub>, c<sub>j</sub>)</code> *such that row* <code>r<sub>i</sub></code> *and column* <code>c<sub>j</sub></code> *are equal*.

A row and column pair is considered equal if they contain the same elements in the same order (i.e., an equal array).

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/06/01/ex1.jpg)
<pre>
<strong>Input:</strong> grid = [[3,2,1],[1,7,6],[2,7,7]]
<strong>Output:</strong> 1
<strong>Explanation:</strong> There is 1 equal row and column pair:
- (Row 2, Column 1): [2,7,7]
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/06/01/ex2.jpg)
<pre>
<strong>Input:</strong> grid = [[3,1,2,2],[1,4,4,5],[2,4,2,2],[2,4,2,2]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> There are 3 equal row and column pairs:
- (Row 0, Column 0): [3,1,2,2]
- (Row 2, Column 2): [2,4,2,2]
- (Row 3, Column 2): [2,4,2,2]
</pre>

#### Constraints:
* `n == grid.length == grid[i].length`
* `1 <= n <= 200`
* <code>1 <= grid[i][j] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = HashMap::new();
        let mut ret = 0;

        for i in 0..grid.len() {
            count
                .entry(grid[i].clone())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        for i in 0..grid.len() {
            let col = (0..grid.len()).map(|j| grid[j][i]).collect::<Vec<_>>();

            ret += count.get(&col).unwrap_or(&0);
        }

        ret
    }
}
```
