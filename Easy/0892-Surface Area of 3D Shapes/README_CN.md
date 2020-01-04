# 892. 三维形体的表面积
在 ```N * N``` 的网格上，我们放置一些 ```1 * 1 * 1```  的立方体。

每个值 ```v = grid[i][j]``` 表示 ```v``` 个正方体叠放在对应单元格 ```(i, j)``` 上。

请你返回最终形体的表面积。

#### 示例 1:
<pre>
<strong>输入:</strong> [[2]]
<strong>输出:</strong> 10
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [[1,2],[3,4]]
<strong>输出:</strong> 34
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> [[1,0],[0,2]]
<strong>输出:</strong> 16
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> [[1,1,1],[1,0,1],[1,1,1]]
<strong>输出:</strong> 32
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> [[2,2,2],[2,1,2],[2,2,2]]
<strong>输出:</strong> 46
</pre>

#### 提示:
* ```1 <= N <= 50```
* ```0 <= grid[i][j] <= 50```

## 题解 (Rust)

### 1. 题解
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
