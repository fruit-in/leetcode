# 2245. Maximum Trailing Zeros in a Cornered Path
You are given a 2D integer array `grid` of size `m x n`, where each cell contains a positive integer.

A **cornered path** is defined as a set of adjacent cells with **at most** one turn. More specifically, the path should exclusively move either **horizontally** or **vertically** up to the turn (if there is one), without returning to a previously visited cell. After the turn, the path will then move exclusively in the **alternate** direction: move vertically if it moved horizontally, and vice versa, also without returning to a previously visited cell.

The **product** of a path is defined as the product of all the values in the path.

Return *the **maximum** number of **trailing zeros** in the product of a cornered path found in* `grid`.

Note:

* **Horizontal** movement means moving in either the left or right direction.
* **Vertical** movement means moving in either the up or down direction.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/03/23/ex1new2.jpg)
<pre>
<strong>Input:</strong> grid = [[23,17,15,3,20],[8,1,20,27,11],[9,4,6,2,21],[40,9,1,10,6],[22,7,4,5,3]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The grid on the left shows a valid cornered path.
It has a product of 15 * 20 * 6 * 1 * 10 = 18000 which has 3 trailing zeros.
It can be shown that this is the maximum trailing zeros in the product of a cornered path.

The grid in the middle is not a cornered path as it has more than one turn.
The grid on the right is not a cornered path as it requires a return to a previously visited cell.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/03/25/ex2.jpg)
<pre>
<strong>Input:</strong> grid = [[4,3,2],[7,6,1],[8,8,8]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> The grid is shown in the figure above.
There are no cornered paths in the grid that result in a product with a trailing zero.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* <code>1 <= m, n <= 10<sup>5</sup></code>
* <code>1 <= m * n <= 10<sup>5</sup></code>
* `1 <= grid[i][j] <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_trailing_zeros(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut count25 = vec![vec![(0, 0); n]; m];
        let mut psud = count25.clone();
        let mut pslr = count25.clone();
        let mut psdu = count25.clone();
        let mut psrl = count25.clone();
        let mut ret = 0;

        for i in 0..m {
            for j in 0..n {
                let mut x = grid[i][j];

                while x % 2 == 0 {
                    count25[i][j].0 += 1;
                    x /= 2;
                }
                while x % 5 == 0 {
                    count25[i][j].1 += 1;
                    x /= 5;
                }

                psud[i][j] = count25[i][j];
                if i > 0 {
                    psud[i][j].0 += psud[i - 1][j].0;
                    psud[i][j].1 += psud[i - 1][j].1;
                }
                pslr[i][j] = count25[i][j];
                if j > 0 {
                    pslr[i][j].0 += pslr[i][j - 1].0;
                    pslr[i][j].1 += pslr[i][j - 1].1;
                }
            }
        }

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                psdu[i][j] = count25[i][j];
                if i < m - 1 {
                    psdu[i][j].0 += psdu[i + 1][j].0;
                    psdu[i][j].1 += psdu[i + 1][j].1;
                }
                psrl[i][j] = count25[i][j];
                if j < n - 1 {
                    psrl[i][j].0 += psrl[i][j + 1].0;
                    psrl[i][j].1 += psrl[i][j + 1].1;
                }

                ret = ret
                    .max(
                        (psud[i][j].0 + pslr[i][j].0 - count25[i][j].0)
                            .min(psud[i][j].1 + pslr[i][j].1 - count25[i][j].1),
                    )
                    .max(
                        (psud[i][j].0 + psrl[i][j].0 - count25[i][j].0)
                            .min(psud[i][j].1 + psrl[i][j].1 - count25[i][j].1),
                    )
                    .max(
                        (psdu[i][j].0 + pslr[i][j].0 - count25[i][j].0)
                            .min(psdu[i][j].1 + pslr[i][j].1 - count25[i][j].1),
                    )
                    .max(
                        (psdu[i][j].0 + psrl[i][j].0 - count25[i][j].0)
                            .min(psdu[i][j].1 + psrl[i][j].1 - count25[i][j].1),
                    );
            }
        }

        ret
    }
}
```
