# 2500. Delete Greatest Value in Each Row
You are given an `m x n` matrix `grid` consisting of positive integers.

Perform the following operation until `grid` becomes empty:

* Delete the element with the greatest value from each row. If multiple such elements exist, delete any of them.
* Add the maximum of deleted elements to the answer.

**Note** that the number of columns decreases by one after each operation.

Return *the answer after performing the operations described above*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/10/19/q1ex1.jpg)
<pre>
<strong>Input:</strong> grid = [[1,2,4],[3,3,1]]
<strong>Output:</strong> 8
<strong>Explanation:</strong> The diagram above shows the removed values in each step.
- In the first operation, we remove 4 from the first row and 3 from the second row (notice that, there are two cells with value 3 and we can remove any of them). We add 4 to the answer.
- In the second operation, we remove 2 from the first row and 3 from the second row. We add 3 to the answer.
- In the third operation, we remove 1 from the first row and 1 from the second row. We add 1 to the answer.
The final answer = 4 + 3 + 1 = 8.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/10/19/q1ex2.jpg)
<pre>
<strong>Input:</strong> grid = [[10]]
<strong>Output:</strong> 10
<strong>Explanation:</strong> The diagram above shows the removed values in each step.
- In the first operation, we remove 10 from the first row. We add 10 to the answer.
The final answer = 10.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 50`
* `1 <= grid[i][j] <= 100`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let m = grid.len();
        let n = grid[0].len();
        let mut ret = 0;

        grid.iter_mut().for_each(|row| row.sort_unstable());

        for col in 0..n {
            ret += (0..m).map(|row| grid[row][col]).max().unwrap();
        }

        ret
    }
}
```
