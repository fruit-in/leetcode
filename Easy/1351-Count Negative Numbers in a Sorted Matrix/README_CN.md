# 1351. 统计有序矩阵中的负数
给你一个 ```m * n``` 的矩阵 ```grid```，矩阵中的元素无论是按行还是按列，都以非递增顺序排列。

请你统计并返回 ```grid``` 中 **负数** 的数目。

#### 示例 1:
<pre>
<strong>输入:</strong> grid = [[4,3,2,-1],[3,2,1,-1],[1,1,-1,-2],[-1,-1,-2,-3]]
<strong>输出:</strong> 8
<strong>解释:</strong> 矩阵中共有 8 个负数。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> grid = [[3,2],[1,0]]
<strong>输出:</strong> 0
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> grid = [[1,-1],[-1,-1]]
<strong>输出:</strong> 3
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> grid = [[-1]]
<strong>输出:</strong> 1
</pre>

#### 提示:
* ```m == grid.length```
* ```n == grid[i].length```
* ```1 <= m, n <= 100```
* ```-100 <= grid[i][j] <= 100```

## 题解 (Rust)

### 1. 二分查找
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
