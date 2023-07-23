# 1861. 旋转盒子
给你一个 `m x n` 的字符矩阵 `box` ，它表示一个箱子的侧视图。箱子的每一个格子可能为：

* `'#'` 表示石头
* `'*'` 表示固定的障碍物
* `'.'` 表示空位置

这个箱子被 **顺时针旋转 90 度** ，由于重力原因，部分石头的位置会发生改变。每个石头会垂直掉落，直到它遇到障碍物，另一个石头或者箱子的底部。重力 **不会** 影响障碍物的位置，同时箱子旋转不会产生**惯性** ，也就是说石头的水平位置不会发生改变。

题目保证初始时 `box` 中的石头要么在一个障碍物上，要么在另一个石头上，要么在箱子的底部。

请你返回一个 `n x m`的矩阵，表示按照上述旋转后，箱子内的结果。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/04/08/rotatingtheboxleetcodewithstones.png)
<pre>
<strong>输入:</strong> box = [["#",".","#"]]
<strong>输出:</strong> [["."],
      ["#"],
      ["#"]]
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/04/08/rotatingtheboxleetcode2withstones.png)
<pre>
<strong>输入:</strong> box = [["#",".","*","."],
            ["#","#","*","."]]
<strong>输出:</strong> [["#","."],
      ["#","#"],
      ["*","*"],
      [".","."]]
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/04/08/rotatingtheboxleetcode3withstone.png)
<pre>
<strong>输入:</strong> box = [["#","#","*",".","*","."],
            ["#","#","#","*",".","."],
            ["#","#","#",".","#","."]]
<strong>输出:</strong> [[".","#","#"],
      [".","#","#"],
      ["#","#","*"],
      ["#","*","."],
      ["#",".","*"],
      ["#",".","."]]
</pre>

#### 提示:
* `m == box.length`
* `n == box[i].length`
* `1 <= m, n <= 500`
* `box[i][j]` 只可能是 `'#'` ，`'*'` 或者 `'.'` 。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn rotate_the_box(bbox: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let m = bbox.len();
        let n = bbox[0].len();
        let mut count = 0;
        let mut ret = vec![vec!['.'; m]; n];

        for i in 0..m {
            for j in 0..n {
                if bbox[i][j] == '#' {
                    count += 1;
                } else if bbox[i][j] == '*' {
                    ret[j][m - i - 1] = '*';

                    for k in 0..count {
                        ret[j - k - 1][m - i - 1] = '#';
                    }
                    count = 0;
                }
            }

            for k in 0..count {
                ret[n - k - 1][m - i - 1] = '#';
            }
            count = 0;
        }

        ret
    }
}
```
