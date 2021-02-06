# 1267. 统计参与通信的服务器
这里有一幅服务器分布图，服务器的位置标识在 `m * n` 的整数矩阵网格 `grid` 中，1 表示单元格上有服务器，0 表示没有。

如果两台服务器位于同一行或者同一列，我们就认为它们之间可以进行通信。

请你统计并返回能够与至少一台其他服务器进行通信的服务器的数量。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/11/24/untitled-diagram-6.jpg)
<pre>
<strong>输入:</strong> grid = [[1,0],[0,1]]
<strong>输出:</strong> 0
<strong>解释:</strong> 没有一台服务器能与其他服务器进行通信。
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/11/24/untitled-diagram-4-1.jpg)
<pre>
<strong>输入:</strong> grid = [[1,0],[1,1]]
<strong>输出:</strong> 3
<strong>解释:</strong> 所有这些服务器都至少可以与一台别的服务器进行通信。
</pre>

#### 示例 3:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/11/24/untitled-diagram-1-3.jpg)
<pre>
<strong>输入:</strong> grid = [[1,1,0,0],[0,0,1,0],[0,0,1,0],[0,0,0,1]]
<strong>输出:</strong> 4
<strong>解释:</strong> 第一行的两台服务器互相通信，第三列的两台服务器互相通信，但右下角的服务器无法与其他服务器通信。
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m <= 250`
* `1 <= n <= 250`
* `grid[i][j] == 0 or 1`

## 题解 (Ruby)

### 1. 题解
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

## 题解 (Rust)

### 1. 题解
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
