# 840. Magic Squares In Grid
A 3 x 3 magic square is a 3 x 3 grid filled with distinct numbers **from 1 to 9** such that each row, column, and both diagonals all have the same sum.

Given an ```grid``` of integers, how many 3 x 3 "magic square" subgrids are there?  (Each subgrid is contiguous).

#### Example 1:
<pre>
<strong>Input:</strong> [[4,3,8,4],
        [9,5,1,9],
        [2,7,6,2]]
<strong>Output:</strong> 1
<strong>Explanation:</strong>
The following subgrid is a 3 x 3 magic square:
438
951
276

while this one is not:
384
519
762

In total, there is only one magic square inside the given grid.
</pre>

#### Note:
1. ```1 <= grid.length <= 10```
2. ```1 <= grid[0].length <= 10```
3. ```0 <= grid[i][j] <= 15```

## Solutions (Rust)

### 1. Brute Force
```Rust
impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let mut ret = 0;

        for i in 2..grid.len() {
            for j in 2..grid[0].len() {
                if grid[i - 1][j - 1] == 5 &&
                   grid[i - 2][j - 2] + grid[i][j] == 10 &&
                   grid[i - 2][j] + grid[i][j - 2] == 10 &&
                   grid[i - 1][j - 2] + grid[i - 1][j] == 10 &&
                   grid[i - 2][j - 1] + grid[i][j - 1] == 10 &&
                   grid[i - 2][j - 2] + grid[i - 2][j - 1] + grid[i - 2][j] == 15 &&
                   grid[i - 2][j - 2] + grid[i - 1][j - 2] + grid[i][j - 2] == 15 {

                    let mut nums = Vec::new();
                    nums.extend_from_slice(&grid[i - 2][(j - 2)..(j + 1)]);
                    nums.extend_from_slice(&grid[i - 1][(j - 2)..(j + 1)]);
                    nums.extend_from_slice(&grid[i][(j - 2)..(j + 1)]);
                    nums.sort_unstable();

                    if nums == vec![1, 2, 3, 4, 5, 6, 7, 8, 9] {
                        ret += 1;
                    }
                }
            }
        }

        ret
    }
}
```
