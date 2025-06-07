# 2033. 获取单值网格的最小操作数
给你一个大小为 `m x n` 的二维整数网格 `grid` 和一个整数 `x` 。每一次操作，你可以对 `grid` 中的任一元素 **加** `x` 或 **减** `x` 。

**单值网格** 是全部元素都相等的网格。

返回使网格化为单值网格所需的 **最小** 操作数。如果不能，返回 `-1` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/09/21/gridtxt.png)
<pre>
<strong>输入:</strong> grid = [[2,4],[6,8]], x = 2
<strong>输出:</strong> 4
<strong>解释:</strong> 可以执行下述操作使所有元素都等于 4 ：
- 2 加 x 一次。
- 6 减 x 一次。
- 8 减 x 两次。
共计 4 次操作。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/09/21/gridtxt-1.png)
<pre>
<strong>输入:</strong> grid = [[1,5],[2,3]], x = 1
<strong>输出:</strong> 5
<strong>解释:</strong> 可以使所有元素都等于 3 。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/09/21/gridtxt-2.png)
<pre>
<strong>输入:</strong> grid = [[1,2],[3,4]], x = 2
<strong>输出:</strong> -1
<strong>解释:</strong> 无法使所有元素相等。
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* <code>1 <= m, n <= 10<sup>5</sup></code>
* <code>1 <= m * n <= 10<sup>5</sup></code>
* <code>1 <= x, grid[i][j] <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut nums = grid.concat();
        let mid = nums.len() / 2;

        if nums.iter().any(|&y| y % x != nums[0] % x) {
            return -1;
        }

        nums.sort_unstable();

        nums.iter().map(|&y| (y - nums[mid]).abs() / x).sum()
    }
}
```
