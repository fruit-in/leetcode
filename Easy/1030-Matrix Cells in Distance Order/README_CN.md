# 1030. 距离顺序排列矩阵单元格
给出 ```R``` 行 ```C``` 列的矩阵，其中的单元格的整数坐标为 ```(r, c)```，满足 ```0 <= r < R``` 且 ```0 <= c < C```。

另外，我们在该矩阵中给出了一个坐标为 ```(r0, c0)``` 的单元格。

返回矩阵中的所有单元格的坐标，并按到 ```(r0, c0)``` 的距离从最小到最大的顺序排，其中，两单元格```(r1, c1)``` 和 ```(r2, c2)``` 之间的距离是曼哈顿距离，```|r1 - r2| + |c1 - c2|```。（你可以按任何满足此条件的顺序返回答案。）

#### 示例 1:
<pre>
<strong>输入:</strong> R = 1, C = 2, r0 = 0, c0 = 0
<strong>输出:</strong> [[0,0],[0,1]]
<strong>解释:</strong> 从 (r0, c0) 到其他单元格的距离为：[0,1]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> R = 2, C = 2, r0 = 0, c0 = 1
<strong>输出:</strong> [[0,1],[0,0],[1,1],[1,0]]
<strong>解释:</strong> 从 (r0, c0) 到其他单元格的距离为：[0,1,1,2]
[[0,1],[1,1],[0,0],[1,0]] 也会被视作正确答案。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> R = 2, C = 3, r0 = 1, c0 = 2
<strong>输出:</strong> [[1,2],[0,2],[1,1],[0,1],[1,0],[0,0]]
<strong>解释:</strong> 从 (r0, c0) 到其他单元格的距离为：[0,1,1,2,2,3]
其他满足题目要求的答案也会被视为正确，例如 [[1,2],[1,1],[0,2],[1,0],[0,1],[0,0]]。
</pre>

#### 提示:
1. ```1 <= R <= 100```
2. ```1 <= C <= 100```
3. ```0 <= r0 < R```
4. ```0 <= c0 < C```

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut cells = Vec::new();
        for i in 0..r {
            for j in 0..c {
                cells.push(vec![i, j]);
            }
        }

        cells.sort_unstable_by_key(|v| (v[0] - r0).abs() + (v[1] - c0).abs());

        cells
    }
}
```

### 2. 根据距离构造矩阵
```Rust
impl Solution {
    pub fn all_cells_dist_order(r: i32, c: i32, r0: i32, c0: i32) -> Vec<Vec<i32>> {
        let mut max_dis = r0.max(r - 1 - r0) + c0.max(c - 1 - c0);
        let mut cells = Vec::new();

        for dis in 0..=max_dis {
            for r_diff in (-dis).max(-r0)..=dis.min(r - 1 - r0) {
                let c_diff = dis - r_diff.abs();

                if c_diff == 0 {
                    cells.push(vec![r0 + r_diff, c0]);
                } else {
                    if c0 - c_diff > -1 {
                        cells.push(vec![r0 + r_diff, c0 - c_diff]);
                    }
                    if c0 + c_diff < c {
                        cells.push(vec![r0 + r_diff, c0 + c_diff]);
                    }
                }
            }
        }

        cells
    }
}
```
