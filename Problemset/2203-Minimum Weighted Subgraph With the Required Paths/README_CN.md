# 2203. 得到要求路径的最小带权子图
给你一个整数 `n` ，它表示一个 **带权有向** 图的节点数，节点编号为 `0` 到 `n - 1` 。

同时给你一个二维整数数组 `edges` ，其中 <code>edges[i] = [from<sub>i</sub>, to<sub>i</sub>, weight<sub>i</sub>]</code> ，表示从 <code>from<sub>i</sub></code> 到 <code>to<sub>i</sub></code> 有一条边权为 <code>weight<sub>i</sub></code> 的 **有向** 边。

最后，给你三个 **互不相同** 的整数 `src1` ，`src2` 和 `dest` ，表示图中三个不同的点。

请你从图中选出一个 **边权和最小** 的子图，使得从 `src1` 和 `src2` 出发，在这个子图中，都 **可以** 到达 `dest` 。如果这样的子图不存在，请返回 `-1` 。

**子图** 中的点和边都应该属于原图的一部分。子图的边权和定义为它所包含的所有边的权值之和。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/02/17/example1drawio.png)
<pre>
<strong>输入:</strong> n = 6, edges = [[0,2,2],[0,5,6],[1,0,3],[1,4,5],[2,1,1],[2,3,3],[2,3,4],[3,4,2],[4,5,1]], src1 = 0, src2 = 1, dest = 5
<strong>输出:</strong> 9
<strong>解释:</strong>
上图为输入的图。
蓝色边为最优子图之一。
注意，子图 [[1,0,3],[0,5,6]] 也能得到最优解，但无法在满足所有限制的前提下，得到更优解。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/02/17/example2-1drawio.png)
<pre>
<strong>输入:</strong> n = 3, edges = [[0,1,1],[2,1,1]], src1 = 0, src2 = 1, dest = 2
<strong>输出:</strong> -1
<strong>解释:</strong>
上图为输入的图。
可以看到，不存在从节点 1 到节点 2 的路径，所以不存在任何子图满足所有限制。
</pre>

#### 提示:
* <code>3 <= n <= 10<sup>5</sup></code>
* <code>0 <= edges.length <= 10<sup>5</sup></code>
* `edges[i].length == 3`
* <code>0 <= from<sub>i</sub>, to<sub>i</sub>, src1, src2, dest <= n - 1</code>
* <code>from<sub>i</sub> != to<sub>i</sub></code>
* `src1` ，`src2` 和 `dest` 两两不同。
* <code>1 <= weight[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_weight(n: i32, edges: Vec<Vec<i32>>, src1: i32, src2: i32, dest: i32) -> i64 {
        let n = n as usize;
        let src1 = src1 as usize;
        let src2 = src2 as usize;
        let dest = dest as usize;
        let mut froms = vec![vec![]; n];
        let mut tos = vec![vec![]; n];
        let mut from_src1 = vec![-1; n];
        let mut from_src2 = vec![-1; n];
        let mut to_dest = vec![-1; n];
        from_src1[src1] = 0;
        from_src2[src2] = 0;
        to_dest[dest] = 0;

        for edge in &edges {
            let (from, to, weight) = (edge[0] as usize, edge[1] as usize, edge[2] as i64);
            froms[to].push((from, weight));
            tos[from].push((to, weight));
        }

        let mut heap = BinaryHeap::from([(Reverse(0), src1)]);
        while let Some((Reverse(weights), from)) = heap.pop() {
            if weights > from_src1[from] {
                continue;
            }

            for &(to, weight) in &tos[from] {
                if from_src1[to] == -1 || weights + weight < from_src1[to] {
                    from_src1[to] = weights + weight;
                    heap.push((Reverse(from_src1[to]), to));
                }
            }
        }

        heap = BinaryHeap::from([(Reverse(0), src2)]);
        while let Some((Reverse(weights), from)) = heap.pop() {
            if weights > from_src2[from] {
                continue;
            }

            for &(to, weight) in &tos[from] {
                if from_src2[to] == -1 || weights + weight < from_src2[to] {
                    from_src2[to] = weights + weight;
                    heap.push((Reverse(from_src2[to]), to));
                }
            }
        }

        heap = BinaryHeap::from([(Reverse(0), dest)]);
        while let Some((Reverse(weights), to)) = heap.pop() {
            if weights > to_dest[to] {
                continue;
            }

            for &(from, weight) in &froms[to] {
                if to_dest[from] == -1 || weights + weight < to_dest[from] {
                    to_dest[from] = weights + weight;
                    heap.push((Reverse(to_dest[from]), from));
                }
            }
        }

        (0..n)
            .filter(|&i| from_src1[i] >= 0 && from_src2[i] >= 0 && to_dest[i] >= 0)
            .map(|i| from_src1[i] + from_src2[i] + to_dest[i])
            .min()
            .unwrap_or(-1)
    }
}
```
