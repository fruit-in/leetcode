# 1260. 二维网格迁移
给你一个 ```n``` 行 ```m``` 列的二维网格 ```grid``` 和一个整数 ```k```。你需要将 ```grid``` 迁移 ```k``` 次。

每次「迁移」操作将会引发下述活动：
* 位于 ```grid[i][j]``` 的元素将会移动到 ```grid[i][j + 1]```。
* 位于 ```grid[i][n - 1]``` 的元素将会移动到 ```grid[i + 1][0]```。
* 位于 ```grid[m - 1][n - 1]``` 的元素将会移动到 ```grid[0][0]```。

请你返回 ```k``` 次迁移操作后最终得到的 **二维网格**。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/11/16/e1-1.png)
![](https://assets.leetcode.com/uploads/2019/11/05/e1.png)
<pre>
<strong>输入:</strong> grid = [[1,2,3],[4,5,6],[7,8,9]], k = 1
<strong>输出:</strong> [[9,1,2],[3,4,5],[6,7,8]]
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/11/16/e2-1.png)
<pre>
<strong>输入:</strong> grid = [[3,8,1,9],[19,7,2,5],[4,6,11,10],[12,0,21,13]], k = 4
<strong>输出:</strong> [[12,0,21,13],[3,8,1,9],[19,7,2,5],[4,6,11,10]]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> grid = [[1,2,3],[4,5,6],[7,8,9]], k = 9
<strong>输出:</strong> [[1,2,3],[4,5,6],[7,8,9]]
</pre>

#### 提示:
* ```1 <= grid.length <= 50```
* ```1 <= grid[i].length <= 50```
* ```-1000 <= grid[i][j] <= 1000```
* ```0 <= k <= 100```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let (m, n) = (grid.len(), grid[0].len());
        let k = (k as usize) % (m * n);
        let mut ret = vec![vec![0; n]; m];

        for pos in 0..(m * n) {
            let tmp = (pos + m * n - k) % (m * n);
            ret[pos / n][pos % n] = grid[tmp / n][tmp % n];
        }

        ret
    }
}
```
