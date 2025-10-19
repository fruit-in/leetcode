# 1727. Largest Submatrix With Rearrangements
You are given a binary matrix `matrix` of size `m x n`, and you are allowed to rearrange the **columns** of the `matrix` in any order.

Return *the area of the largest submatrix within* `matrix` *where **every** element of the submatrix is* `1` *after reordering the columns optimally*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/12/29/screenshot-2020-12-30-at-40536-pm.png)
<pre>
<strong>Input:</strong> matrix = [[0,0,1],[1,1,1],[1,0,1]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> You can rearrange the columns as shown above.
The largest submatrix of 1s, in bold, has an area of 4.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/12/29/screenshot-2020-12-30-at-40852-pm.png)
<pre>
<strong>Input:</strong> matrix = [[1,0,1,0,1]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> You can rearrange the columns as shown above.
The largest submatrix of 1s, in bold, has an area of 3.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> matrix = [[1,1,0],[1,0,1]]
<strong>Output:</strong> 2
<strong>Explanation:</strong> Notice that you must rearrange entire columns, and there is no way to make a submatrix of 1s larger than an area of 2.
</pre>

#### Constraints:
* `m == matrix.length`
* `n == matrix[i].length`
* <code>1 <= m * n <= 10<sup>5</sup></code>
* `matrix[i][j]` is either `0` or `1`.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut consecutive_ones = vec![0; n];
        let mut ret = 0;

        for r in 0..m {
            let mut heap = BinaryHeap::new();

            for c in 0..n {
                consecutive_ones[c] = (consecutive_ones[c] + 1) * matrix[r][c];
                if consecutive_ones[c] > 0 {
                    heap.push(consecutive_ones[c]);
                }
            }

            for length in 1..=heap.len() as i32 {
                ret = ret.max(length * heap.pop().unwrap());
            }
        }

        ret
    }
}
```
