# 684. 冗余连接
树可以看成是一个连通且 **无环** 的 **无向** 图。

给定一个图，该图从一棵 `n` 个节点 (节点值 `1～n`) 的树中添加一条边后获得。添加的边的两个不同顶点编号在 `1` 到 `n` 中间，且这条附加的边不属于树中已存在的边。图的信息记录于长度为 `n` 的二维数组 `edges` ，<code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> 表示图中在 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 之间存在一条边。

请找出一条可以删去的边，删除后可使得剩余部分是一个有着 `n` 个节点的树。如果有多个答案，则返回数组 `edges` 中最后出现的那个。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/05/02/reduntant1-1-graph.jpg)
<pre>
<strong>输入:</strong> edges = [[1,2],[1,3],[2,3]]
<strong>输出:</strong> [2,3]
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/05/02/reduntant1-2-graph.jpg)
<pre>
<strong>输入:</strong> edges = [[1,2],[2,3],[3,4],[1,4],[1,5]]
<strong>输出:</strong> [1,4]
</pre>

#### 提示:
* `n == edges.length`
* `3 <= n <= 1000`
* `edges[i].length == 2`
* <code>1 <= a<sub>i</sub> < b<sub>i</sub> <= edges.length</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* `edges` 中无重复元素
* 给定的图是连通的

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();

        for i in (0..n).rev() {
            let mut neighbors = vec![vec![]; n];
            let mut stack = vec![(n, 0)];
            let mut visited = vec![false; n];
            visited[0] = true;

            for j in 0..n {
                if j != i {
                    let (a, b) = (edges[j][0] as usize - 1, edges[j][1] as usize - 1);
                    neighbors[a].push(b);
                    neighbors[b].push(a);
                }
            }

            while let Some((from, to)) = stack.pop() {
                for &j in &neighbors[to] {
                    if j != from {
                        if visited[j] {
                            stack = vec![];
                            visited[0] = false;
                            break;
                        } else {
                            stack.push((to, j));
                            visited[j] = true;
                        }
                    }
                }
            }

            if visited.iter().all(|&v| v) {
                return edges[i].clone();
            }
        }

        unreachable!()
    }
}
```
