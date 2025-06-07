# 2033. Minimum Operations to Make a Uni-Value Grid
You are given a 2D integer `grid` of size `m x n` and an integer `x`. In one operation, you can **add** `x` to or **subtract** `x` from any element in the `grid`.

A **uni-value grid** is a grid where all the elements of it are equal.

Return *the **minimum** number of operations to make the grid **uni-value***. If it is not possible, return `-1`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/09/21/gridtxt.png)
<pre>
<strong>Input:</strong> grid = [[2,4],[6,8]], x = 2
<strong>Output:</strong> 4
<strong>Explanation:</strong> We can make every element equal to 4 by doing the following:
- Add x to 2 once.
- Subtract x from 6 once.
- Subtract x from 8 twice.
A total of 4 operations were used.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/09/21/gridtxt-1.png)
<pre>
<strong>Input:</strong> grid = [[1,5],[2,3]], x = 1
<strong>Output:</strong> 5
<strong>Explanation:</strong> We can make every element equal to 3.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/09/21/gridtxt-2.png)
<pre>
<strong>Input:</strong> grid = [[1,2],[3,4]], x = 2
<strong>Output:</strong> -1
<strong>Explanation:</strong> It is impossible to make every element equal.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* <code>1 <= m, n <= 10<sup>5</sup></code>
* <code>1 <= m * n <= 10<sup>5</sup></code>
* <code>1 <= x, grid[i][j] <= 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut nums = grid.concat();
        let mid = nums.len() / 2;

        if nums.iter().any(|&y| y % x != nums[0] % x) {
            return -1;
        }

        nums.sort_unstable();

        nums.iter().map(|&y| (y - nums[mid]).abs() / x).sum()
    }
}
```
