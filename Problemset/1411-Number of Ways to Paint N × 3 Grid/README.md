# 1411. Number of Ways to Paint N × 3 Grid
You have a `grid` of size `n x 3` and you want to paint each cell of the grid with exactly one of the three colors: **Red**, **Yellow**, or **Green** while making sure that no two adjacent cells have the same color (i.e., no two cells that share vertical or horizontal sides have the same color).

Given `n` the number of rows of the grid, return *the number of ways* you can paint this `grid`. As the answer may grow large, the answer **must be** computed modulo <code>10<sup>9</sup> + 7</code>.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/03/26/e1.png)
<pre>
<strong>Input:</strong> n = 1
<strong>Output:</strong> 12
<strong>Explanation:</strong> There are 12 possible way to paint the grid as shown.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 5000
<strong>Output:</strong> 30228214
</pre>

#### Constraints:
* `n == grid.length`
* `1 <= n <= 5000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let mut x = 6;
        let mut y = 6;

        for _ in 1..n {
            x = (x + y) % 1_000_000_007 * 2 % 1_000_000_007;
            y = (x + y) % 1_000_000_007;
        }

        (x + y) % 1_000_000_007
    }
}
```
