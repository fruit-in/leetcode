# 994. Rotting Oranges
In a given grid, each cell can have one of three values:
* the value ```0``` representing an empty cell;
* the value ```1``` representing a fresh orange;
* the value ```2``` representing a rotten orange.

Every minute, any fresh orange that is adjacent (4-directionally) to a rotten orange becomes rotten.

Return the minimum number of minutes that must elapse until no cell has a fresh orange.  If this is impossible, return ```-1``` instead.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/02/16/oranges.png)
<pre>
<strong>Input:</strong> [[2,1,1],[1,1,0],[0,1,1]]
<strong>Output:</strong> 4
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [[2,1,1],[0,1,1],[1,0,1]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> The orange in the bottom left corner (row 2, column 0) is never rotten, because rotting only happens 4-directionally.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> [[0,2]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> Since there are already no fresh oranges at minute 0, the answer is just 0.
</pre>

#### Note:
1. ```1 <= grid.length <= 10```
2. ```1 <= grid[0].length <= 10```
3. ```grid[i][j]``` is only ```0```, ```1```, or ```2```.

## Solutions (Rust)

### 1. Simulation
```Rust
impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut prev = grid;

        for minute in 0..59 {
            let mut no_fresh = true;
            let mut new = prev.clone();

            for j in 0..prev.len() {
                for k in 0..prev[0].len() {
                    match prev[j][k] {
                        1 => no_fresh = false,
                        2 => {
                            if j > 0 && prev[j - 1][k] == 1 {
                                new[j - 1][k] = 2;
                            }
                            if k > 0 && prev[j][k - 1] == 1 {
                                new[j][k - 1] = 2;
                            }
                            if j < prev.len() - 1 && prev[j + 1][k] == 1 {
                                new[j + 1][k] = 2;
                            }
                            if k < prev[0].len() - 1 && prev[j][k + 1] == 1 {
                                new[j][k + 1] = 2;
                            }
                        },
                        _ => (),
                    };
                }
            }

            if no_fresh {
                return minute;
            }

            prev = new;
        }

        -1
    }
}
```
