# 2639. Find the Width of Columns of a Grid
You are given a **0-indexed** `m x n` integer matrix `grid`. The width of a column is the maximum **length** of its integers.

* For example, if `grid = [[-10], [3], [12]]`, the width of the only column is `3` since `-10` is of length `3`.

Return *an integer array* `ans` *of size* `n` *where* `ans[i]` *is the width of the* <code>i<sup>th</sup></code> *column*.

The **length** of an integer `x` with `len` digits is equal to `len` if `x` is non-negative, and `len + 1` otherwise.

#### Example 1:
<pre>
<strong>Input:</strong> grid = [[1],[22],[333]]
<strong>Output:</strong> [3]
<strong>Explanation:</strong> In the 0th column, 333 is of length 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> grid = [[-15,1,3],[15,7,12],[5,6,-2]]
<strong>Output:</strong> [3,1,2]
<strong>Explanation:</strong>
In the 0th column, only -15 is of length 3.
In the 1st column, all integers are of length 1.
In the 2nd column, both 12 and -2 are of length 2.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 100`
* <code>-10<sup>9</sup> <= grid[r][c] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (grid.len(), grid[0].len());
        let mut ans = vec![];

        for i in 0..n {
            let max_num = (0..m).map(|j| grid[j][i]).max().unwrap();
            let min_num = (0..m).map(|j| grid[j][i]).min().unwrap();

            ans.push(max_num.to_string().len().max(min_num.to_string().len()) as i32);
        }

        ans
    }
}
```
