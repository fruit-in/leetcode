# 883. 三维形体投影面积
在 ```N * N``` 的网格中，我们放置了一些与 x，y，z 三轴对齐的 ```1 * 1 * 1``` 立方体。

每个值 ```v = grid[i][j]``` 表示 ```v``` 个正方体叠放在单元格 ```(i, j)``` 上。

现在，我们查看这些立方体在 xy、yz 和 zx 平面上的*投影*。

投影就像影子，将三维形体映射到一个二维平面上。

在这里，从顶部、前面和侧面看立方体时，我们会看到“影子”。

返回所有三个投影的总面积。

#### 示例 1:
<pre>
<strong>输入:</strong> [[2]]
<strong>输出:</strong> 5
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [[1,2],[3,4]]
<strong>输出:</strong> 17
<strong>解释:</strong>
这里有该形体在三个轴对齐平面上的三个投影(“阴影部分”)。
<img src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/08/02/shadow.png">
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> [[1,0],[0,2]]
<strong>输出:</strong> 8
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> [[1,1,1],[1,0,1],[1,1,1]]
<strong>输出:</strong> 14
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> [[2,2,2],[2,1,2],[2,2,2]]
<strong>输出:</strong> 21
</pre>

#### 提示:
* ```1 <= grid.length = grid[0].length <= 50```
* ```0 <= grid[i][j] <= 50```

## 题解 (Rust)

### 1. 数学
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
