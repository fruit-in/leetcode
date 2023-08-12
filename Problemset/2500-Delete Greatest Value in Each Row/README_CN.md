# 2500. 删除每行中的最大值
给你一个 `m x n` 大小的矩阵 `grid` ，由若干正整数组成。

执行下述操作，直到 `grid` 变为空矩阵：

* 从每一行删除值最大的元素。如果存在多个这样的值，删除其中任何一个。
* 将删除元素中的最大值与答案相加。

**注意** 每执行一次操作，矩阵中列的数据就会减 1 。

返回执行上述操作后的答案。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/10/19/q1ex1.jpg)
<pre>
<strong>输入:</strong> grid = [[1,2,4],[3,3,1]]
<strong>输出:</strong> 8
<strong>解释:</strong> 上图展示在每一步中需要移除的值。
- 在第一步操作中，从第一行删除 4 ，从第二行删除 3（注意，有两个单元格中的值为 3 ，我们可以删除任一）。在答案上加 4 。
- 在第二步操作中，从第一行删除 2 ，从第二行删除 3 。在答案上加 3 。
- 在第三步操作中，从第一行删除 1 ，从第二行删除 1 。在答案上加 1 。
最终，答案 = 4 + 3 + 1 = 8 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/10/19/q1ex2.jpg)
<pre>
<strong>输入:</strong> grid = [[10]]
<strong>输出:</strong> 10
<strong>解释:</strong> 上图展示在每一步中需要移除的值。
- 在第一步操作中，从第一行删除 10 。在答案上加 10 。
最终，答案 = 10 。
</pre>

#### 提示:
* `m == grid.length`
* `n == grid[i].length`
* `1 <= m, n <= 50`
* `1 <= grid[i][j] <= 100`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let m = grid.len();
        let n = grid[0].len();
        let mut ret = 0;

        grid.iter_mut().for_each(|row| row.sort_unstable());

        for col in 0..n {
            ret += (0..m).map(|row| grid[row][col]).max().unwrap();
        }

        ret
    }
}
```
