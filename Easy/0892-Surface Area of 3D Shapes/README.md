# 892. Surface Area of 3D Shapes
On a ```N * N``` grid, we place some ```1 * 1 * 1``` cubes.

Each value ```v = grid[i][j]``` represents a tower of ```v``` cubes placed on top of grid cell ```(i, j)```.

Return the total surface area of the resulting shapes.

#### Example 1:
<pre>
<strong>Input:</strong> [[2]]
<strong>Output:</strong> 10
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [[1,2],[3,4]]
<strong>Output:</strong> 34
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> [[1,0],[0,2]]
<strong>Output:</strong> 16
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> [[1,1,1],[1,0,1],[1,1,1]]
<strong>Output:</strong> 32
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> [[2,2,2],[2,1,2],[2,2,2]]
<strong>Output:</strong> 46
</pre>

#### Note:
* ```1 <= N <= 50```
* ```0 <= grid[i][j] <= 50```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut area = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] > 0 {
                    area += grid[i][j] * 2 + 1;
                    if i > 0 {
                        area -= grid[i - 1][j].min(grid[i][j]);
                    }
                    if j > 0 {
                        area -= grid[i][j - 1].min(grid[i][j]);
                    }
                }
            }
        }

        area * 2
    }
}
```
