# 2201. 统计可以提取的工件
存在一个 `n x n` 大小、下标从 **0** 开始的网格，网格中埋着一些工件。给你一个整数 `n` 和一个下标从 **0** 开始的二维整数数组 `artifacts` ，`artifacts` 描述了矩形工件的位置，其中 <code>artifacts[i] = [r1<sub>i</sub>, c1<sub>i</sub>, r2<sub>i</sub>, c2<sub>i</sub>]</code> 表示第 `i` 个工件在子网格中的填埋情况：

* <code>(r1<sub>i</sub>, c1<sub>i</sub>)</code> 是第 `i` 个工件 **左上** 单元格的坐标，且
* <code>(r2<sub>i</sub>, c2<sub>i</sub>)</code> 是第 `i` 个工件 **右下** 单元格的坐标。

你将会挖掘网格中的一些单元格，并清除其中的填埋物。如果单元格中埋着工件的一部分，那么该工件这一部分将会裸露出来。如果一个工件的所有部分都都裸露出来，你就可以提取该工件。

给你一个下标从 **0** 开始的二维整数数组 `dig` ，其中 <code>dig[i] = [r<sub>i</sub>, c<sub>i</sub>]</code> 表示你将会挖掘单元格 <code>(r<sub>i</sub>, c<sub>i</sub>)</code> ，返回你可以提取的工件数目。

生成的测试用例满足：

* 不存在重叠的两个工件。
* 每个工件最多只覆盖 `4` 个单元格。
* `dig` 中的元素互不相同。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2019/09/16/untitled-diagram.jpg)
<pre>
<strong>输入:</strong> n = 2, artifacts = [[0,0,0,0],[0,1,1,1]], dig = [[0,0],[0,1]]
<strong>输出:</strong> 1
<strong>解释:</strong>
不同颜色表示不同的工件。挖掘的单元格用 'D' 在网格中进行标记。
有 1 个工件可以提取，即红色工件。
蓝色工件在单元格 (1,1) 的部分尚未裸露出来，所以无法提取该工件。
因此，返回 1 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2019/09/16/untitled-diagram-1.jpg)
<pre>
<strong>输入:</strong> n = 2, artifacts = [[0,0,0,0],[0,1,1,1]], dig = [[0,0],[0,1],[1,1]]
<strong>输出:</strong> 2
<strong>解释:</strong> 红色工件和蓝色工件的所有部分都裸露出来（用 'D' 标记），都可以提取。因此，返回 2 。
</pre>

#### 提示:
* `1 <= n <= 1000`
* <code>1 <= artifacts.length, dig.length <= min(n<sup>2</sup>, 10<sup>5</sup>)</code>
* `artifacts[i].length == 4`
* `dig[i].length == 2`
* <code>0 <= r1<sub>i</sub>, c1<sub>i</sub>, r2<sub>i</sub>, c2<sub>i</sub>, r<sub>i</sub>, c<sub>i</sub> <= n - 1</code>
* <code>r1<sub>i</sub> <= r2<sub>i</sub></code>
* <code>c1<sub>i</sub> <= c2<sub>i</sub></code>
* 不存在重叠的两个工件
* 每个工件 **最多** 只覆盖 `4` 个单元格
* `dig` 中的元素互不相同

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn dig_artifacts(n: i32, artifacts: Vec<Vec<i32>>, dig: Vec<Vec<i32>>) -> i32 {
        let mut grid = vec![vec![false; n as usize]; n as usize];
        let mut ret = 0;

        for d in &dig {
            grid[d[0] as usize][d[1] as usize] = true;
        }

        for artifact in &artifacts {
            let mut flag = true;

            for r in artifact[0] as usize..=artifact[2] as usize {
                for c in artifact[1] as usize..=artifact[3] as usize {
                    flag &= grid[r][c];
                }
            }

            ret += flag as i32;
        }

        ret
    }
}
```
