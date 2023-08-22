# 1706. 球会落何处
用一个大小为 `m x n` 的二维网格 `grid` 表示一个箱子。你有 `n` 颗球。箱子的顶部和底部都是开着的。

箱子中的每个单元格都有一个对角线挡板，跨过单元格的两个角，可以将球导向左侧或者右侧。

* 将球导向右侧的挡板跨过左上角和右下角，在网格中用 `1` 表示。
* 将球导向左侧的挡板跨过右上角和左下角，在网格中用 `-1` 表示。

在箱子每一列的顶端各放一颗球。每颗球都可能卡在箱子里或从底部掉出来。如果球恰好卡在两块挡板之间的 "V" 形图案，或者被一块挡导向到箱子的任意一侧边上，就会卡住。

返回一个大小为 `n` 的数组 `answer` ，其中 `answer[i]` 是球放在顶部的第 `i` 列后从底部掉出来的那一列对应的下标，如果球卡在盒子里，则返回 `-1` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2019/09/26/ball.jpg)
<pre>
<strong>输入:</strong> grid = [[1,1,1,-1,-1],[1,1,1,-1,-1],[-1,-1,-1,1,1],[1,1,1,1,-1],[-1,-1,-1,-1,-1]]
<strong>输出:</strong> [1,-1,-1,-1,-1]
<strong>解释:</strong> 示例如图：
b0 球开始放在第 0 列上，最终从箱子底部第 1 列掉出。
b1 球开始放在第 1 列上，会卡在第 2、3 列和第 1 行之间的 "V" 形里。
b2 球开始放在第 2 列上，会卡在第 2、3 列和第 0 行之间的 "V" 形里。
b3 球开始放在第 3 列上，会卡在第 2、3 列和第 0 行之间的 "V" 形里。
b4 球开始放在第 4 列上，会卡在第 2、3 列和第 1 行之间的 "V" 形里。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> grid = [[-1]]
<strong>输出:</strong> [-1]
<strong>解释:</strong> 球被卡在箱子左侧边上。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> grid = [[1,1,1,1,1,1],[-1,-1,-1,-1,-1,-1],[1,1,1,1,1,1],[-1,-1,-1,-1,-1,-1]]
<strong>输出:</strong> [0,1,2,3,4,-1]
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 100`
* `grid[i][j]` 为 `1` 或 `-1`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid[0].len();
        let mut answer = (0..n as i32).collect::<Vec<_>>();

        for row in &grid {
            for ball in 0..n {
                if answer[ball] != -1 {
                    if row[answer[ball] as usize] == 1
                        && answer[ball] + 1 < n as i32
                        && row[answer[ball] as usize + 1] == 1
                    {
                        answer[ball] += 1;
                    } else if row[answer[ball] as usize] == -1
                        && answer[ball] > 0
                        && row[answer[ball] as usize - 1] == -1
                    {
                        answer[ball] -= 1;
                    } else {
                        answer[ball] = -1;
                    }
                }
            }
        }

        answer
    }
}
```
