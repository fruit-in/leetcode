# 1001. 网格照明
在大小为 `n x n` 的网格 `grid` 上，每个单元格都有一盏灯，最初灯都处于 **关闭** 状态。

给你一个由灯的位置组成的二维数组 `lamps` ，其中 <code>lamps[i] = [row<sub>i<sub>, col<sub>i<sub>]</code> 表示 **打开** 位于 <code>grid[row<sub>i<sub>][col<sub>i<sub>]</code> 的灯。即便同一盏灯可能在 `lamps` 中多次列出，不会影响这盏灯处于 **打开** 状态。

当一盏灯处于打开状态，它将会照亮 **自身所在单元格** 以及同一 **行** 、同一 **列** 和两条 **对角线** 上的 **所有其他单元格** 。

另给你一个二维数组 `queries` ，其中 <code>queries[j] = [row<sub>j</sub>, col<sub>j</sub>]</code> 。对于第 `j` 个查询，如果单元格 <code>[row<sub>j</sub>, col<sub>j</sub>]</code> 是被照亮的，则查询结果为 `1` ，否则为 `0` 。在第 `j` 次查询之后 [按照查询的顺序] ，关闭 位于单元格 <code>grid[row<sub>j</sub>][col<sub>j</sub>]</code> 上及相邻 8 个方向上（与单元格 <code>grid[row<sub>i<sub>][col<sub>i<sub>]</code> 共享角或边）的任何灯。

返回一个整数数组 `ans` 作为答案， `ans[j]` 应等于第 `j` 次查询 `queries[j]` 的结果，`1` 表示照亮，`0` 表示未照亮。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/08/19/illu_1.jpg)
<pre>
<strong>输入:</strong> n = 5, lamps = [[0,0],[4,4]], queries = [[1,1],[1,0]]
<strong>输出:</strong> [1,0]
<strong>解释:</strong> 最初所有灯都是关闭的。在执行查询之前，打开位于 [0, 0] 和 [4, 4] 的灯。第 0 次查询检查 grid[1][1] 是否被照亮（蓝色方框）。该单元格被照亮，所以 ans[0] = 1 。然后，关闭红色方框中的所有灯。
<img src="https://assets.leetcode.com/uploads/2020/08/19/illu_step1.jpg">
第 1 次查询检查 grid[1][0] 是否被照亮（蓝色方框）。该单元格没有被照亮，所以 ans[1] = 0 。然后，关闭红色矩形中的所有灯。
<img src="https://assets.leetcode.com/uploads/2020/08/19/illu_step2.jpg">
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 5, lamps = [[0,0],[4,4]], queries = [[1,1],[1,1]]
<strong>输出:</strong> [1,1]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 5, lamps = [[0,0],[0,4]], queries = [[0,4],[0,1],[1,4]]
<strong>输出:</strong> [1,1,0]
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>9</sup></code>
* `0 <= lamps.length <= 20000`
* `0 <= queries.length <= 20000`
* `lamps[i].length == 2`
* <code>0 <= row<sub>i</sub>, col<sub>i</sub> < n</code>
* `queries[j].length == 2`
* <code>0 <= row<sub>j</sub>, col<sub>j</sub> < n</code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut on_lamps = HashMap::new();
        let mut illuminated_rows = HashMap::new();
        let mut illuminated_cols = HashMap::new();
        let mut illuminated_dias0 = HashMap::new();
        let mut illuminated_dias1 = HashMap::new();
        let mut ans = vec![0; queries.len()];

        for lamp in &lamps {
            let row = lamp[0];
            let col = lamp[1];
            let dia0 = row - col;
            let dia1 = row + col;

            if on_lamps.insert((row, col), (dia0, dia1)).is_none() {
                illuminated_rows
                    .entry(row)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                illuminated_cols
                    .entry(col)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                illuminated_dias0
                    .entry(dia0)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                illuminated_dias1
                    .entry(dia1)
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }

        for i in 0..queries.len() {
            let row = queries[i][0];
            let col = queries[i][1];
            let dia0 = row - col;
            let dia1 = row + col;

            ans[i] = (*illuminated_rows.get(&row).unwrap_or(&0) > 0
                || *illuminated_cols.get(&col).unwrap_or(&0) > 0
                || *illuminated_dias0.get(&dia0).unwrap_or(&0) > 0
                || *illuminated_dias1.get(&dia1).unwrap_or(&0) > 0) as i32;

            for x in -1..2 {
                for y in -1..2 {
                    if let Some((dia0, dia1)) = on_lamps.remove(&(row + x, col + y)) {
                        *illuminated_rows.get_mut(&(row + x)).unwrap() -= 1;
                        *illuminated_cols.get_mut(&(col + y)).unwrap() -= 1;
                        *illuminated_dias0.get_mut(&dia0).unwrap() -= 1;
                        *illuminated_dias1.get_mut(&dia1).unwrap() -= 1;
                    }
                }
            }
        }

        ans
    }
}
```
