# 1351. Count Negative Numbers in a Sorted Matrix
Given a ```m * n``` matrix ```grid``` which is sorted in non-increasing order both row-wise and column-wise.

Return the number of **negative** numbers in ```grid```.

#### Example 1:
<pre>
<strong>Input:</strong> grid = [[4,3,2,-1],[3,2,1,-1],[1,1,-1,-2],[-1,-1,-2,-3]]
<strong>Output:</strong> 8
<strong>Explanation:</strong> There are 8 negatives number in the matrix.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> grid = [[3,2],[1,0]]
<strong>Output:</strong> 0
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> grid = [[1,-1],[-1,-1]]
<strong>Output:</strong> 3
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> grid = [[-1]]
<strong>Output:</strong> 1
</pre>

#### Constraints:
* ```m == grid.length```
* ```n == grid[i].length```
* ```1 <= m, n <= 100```
* ```-100 <= grid[i][j] <= 100```

## Solutions (Rust)

### 1. Binary Search
```Rust
impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut pos = 0;

        for row in grid {
            match row.binary_search_by(|probe| 0.cmp(&probe)) {
                Ok(i) => pos += i + 1,
                Err(i) => pos += i,
            }
        }

        (m * n - pos) as i32
    }
}
```
