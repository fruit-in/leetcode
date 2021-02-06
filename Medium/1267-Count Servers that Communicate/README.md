# 1267. Count Servers that Communicate
You are given a map of a server center, represented as a `m * n` integer matrix `grid`, where 1 means that on that cell there is a server and 0 means that it is no server. Two servers are said to communicate if they are on the same row or on the same column.

Return the number of servers that communicate with any other server.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/11/14/untitled-diagram-6.jpg)
<pre>
<strong>Input:</strong> grid = [[1,0],[0,1]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> No servers can communicate with others.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2019/11/13/untitled-diagram-4.jpg)
<pre>
<strong>Input:</strong> grid = [[1,0],[1,1]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> All three servers can communicate with at least one other server.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2019/11/14/untitled-diagram-1-3.jpg)
<pre>
<strong>Input:</strong> grid = [[1,1,0,0],[0,0,1,0],[0,0,1,0],[0,0,0,1]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The two servers in the first row can communicate with each other. The two servers in the third column can communicate with each other. The server at right bottom corner can't communicate with any other server.
</pre>

#### Constraints:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m <= 250`
* `1 <= n <= 250`
* `grid[i][j] == 0 or 1`

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[][]} grid
# @return {Integer}
def count_servers(grid)
  row_count = [0] * grid.length
  col_count = [0] * grid[0].length
  ret = 0

  (0...grid.length).each do |r|
    (0...grid[0].length).each do |c|
      if grid[r][c] == 1
        row_count[r] += 1
        col_count[c] += 1
      end
    end
  end

  (0...grid.length).each do |r|
    (0...grid[0].length).each do |c|
      ret += 1 if grid[r][c] == 1 && (row_count[r] > 1 || col_count[c] > 1)
    end
  end

  ret
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut row_count = vec![0; grid.len()];
        let mut col_count = vec![0; grid[0].len()];
        let mut ret = 0;

        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == 1 {
                    row_count[r] += 1;
                    col_count[c] += 1;
                }
            }
        }

        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == 1 && (row_count[r] > 1 || col_count[c] > 1) {
                    ret += 1;
                }
            }
        }

        ret
    }
}
```
