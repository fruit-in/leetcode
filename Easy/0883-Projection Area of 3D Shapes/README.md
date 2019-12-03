# 883. Projection Area of 3D Shapes
On a ```N * N``` grid, we place some ```1 * 1 * 1``` cubes that are axis-aligned with the x, y, and z axes.

Each value ```v = grid[i][j]``` represents a tower of ```v``` cubes placed on top of grid cell ```(i, j)```.

Now we view the *projection* of these cubes onto the xy, yz, and zx planes.

A projection is like a shadow, that maps our 3 dimensional figure to a 2 dimensional plane.

Here, we are viewing the "shadow" when looking at the cubes from the top, the front, and the side.

Return the total area of all three projections.

#### Example 1:
<pre>
<strong>Input:</strong> [[2]]
<strong>Output:</strong> 5
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [[1,2],[3,4]]
<strong>Output:</strong> 17
<strong>Explanation:</strong>
Here are the three projections ("shadows") of the shape made with each axis-aligned plane.
<img src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/08/02/shadow.png">
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> [[1,0],[0,2]]
<strong>Output:</strong> 8
<strong>Explanation:</strong> 
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> [[1,1,1],[1,0,1],[1,1,1]]
<strong>Output:</strong> 14
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> [[2,2,2],[2,1,2],[2,2,2]]
<strong>Output:</strong> 21
</pre>

#### Note:
* ```1 <= grid.length = grid[0].length <= 50```
* ```0 <= grid[i][j] <= 50```

## Solutions (Rust)

### 1. Mathematical
```Rust
impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut top = 0;
        let mut front = 0;
        let mut side = 0;

        for x in 0..grid.len() {
            let mut front_max = 0;
            let mut side_max = 0;

            for y in 0..grid[0].len() {
                if grid[x][y] > 0 {
                    top += 1;
                }
                front_max = front_max.max(grid[x][y]);
                side_max = side_max.max(grid[y][x]);
            }

            front += front_max;
            side += side_max;
        }

        top + front + side
    }
}
```
