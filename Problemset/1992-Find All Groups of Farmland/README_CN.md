# 1992. 找到所有的农场组
给你一个下标从 **0** 开始，大小为 `m x n` 的二进制矩阵 `land` ，其中 `0` 表示一单位的森林土地，`1` 表示一单位的农场土地。

为了让农场保持有序，农场土地之间以矩形的 **农场组** 的形式存在。每一个农场组都 **仅** 包含农场土地。且题目保证不会有两个农场组相邻，也就是说一个农场组中的任何一块土地都 **不会** 与另一个农场组的任何一块土地在四个方向上相邻。

`land` 可以用坐标系统表示，其中 `land` 左上角坐标为 `(0, 0)` ，右下角坐标为 `(m-1, n-1)` 。请你找到所有 **农场组** 最左上角和最右下角的坐标。一个左上角坐标为 <code>(r<sub>1</sub>, c<sub>1</sub>)</code> 且右下角坐标为 <code>(r<sub>2</sub>, c<sub>2</sub>)</code> 的 **农场组** 用长度为 4 的数组 <code>[r<sub>1</sub>, c<sub>1</sub>, r<sub>2</sub>, c<sub>2</sub>]</code> 表示。

请你返回一个二维数组，它包含若干个长度为 4 的子数组，每个子数组表示 `land` 中的一个 **农场组** 。如果没有任何农场组，请你返回一个空数组。可以以 **任意顺序** 返回所有农场组。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/07/27/screenshot-2021-07-27-at-12-23-15-copy-of-diagram-drawio-diagrams-net.png)
<pre>
<strong>输入:</strong> land = [[1,0,0],[0,1,1],[0,1,1]]
<strong>输出:</strong> [[0,0,0,0],[1,1,2,2]]
<strong>解释:</strong>
第一个农场组的左上角为 land[0][0] ，右下角为 land[0][0] 。
第二个农场组的左上角为 land[1][1] ，右下角为 land[2][2] 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/07/27/screenshot-2021-07-27-at-12-30-26-copy-of-diagram-drawio-diagrams-net.png)
<pre>
<strong>输入:</strong> land = [[1,1],[1,1]]
<strong>输出:</strong> [[0,0,1,1]]
<strong>解释:</strong>
第一个农场组左上角为 land[0][0] ，右下角为 land[1][1] 。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/07/27/screenshot-2021-07-27-at-12-32-24-copy-of-diagram-drawio-diagrams-net.png)
<pre>
<strong>输入:</strong> land = [[0]]
<strong>输出:</strong> []
<strong>解释:</strong>
没有任何农场组。
</pre>

#### 提示:
* `m == land.length`
* `n == land[i].length`
* `1 <= m, n <= 300`
* `land` 只包含 `0` 和 `1` 。
* 农场组都是 **矩形** 的形状。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = land.len();
        let n = land[0].len();
        let mut ret = vec![];

        for r1 in 0..m {
            for c1 in 0..n {
                if land[r1][c1] == 0
                    || (r1 > 0 && land[r1 - 1][c1] == 1)
                    || (c1 > 0 && land[r1][c1 - 1] == 1)
                {
                    continue;
                }

                let mut group = vec![r1 as i32, c1 as i32, r1 as i32, c1 as i32];

                for r2 in r1..m {
                    if land[r2][c1] == 1 {
                        group[2] = r2 as i32;
                    } else {
                        break;
                    }
                }
                for c2 in c1..n {
                    if land[group[2] as usize][c2] == 1 {
                        group[3] = c2 as i32;
                    } else {
                        break;
                    }
                }

                ret.push(group);
            }
        }

        ret
    }
}
```
