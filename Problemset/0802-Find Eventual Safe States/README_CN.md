# 802. 找到最终的安全状态
有一个有 `n` 个节点的有向图，节点按 `0` 到 `n - 1` 编号。图由一个 **索引从 0 开始** 的 2D 整数数组 `graph`表示， `graph[i]`是与节点 `i` 相邻的节点的整数数组，这意味着从节点 `i` 到 `graph[i]`中的每个节点都有一条边。

如果一个节点没有连出的有向边，则该节点是 **终端节点** 。如果从该节点开始的所有可能路径都通向 **终端节点** ，则该节点为 **终端节点**（或另一个安全节点）。

返回一个由图中所有 **安全节点** 组成的数组作为答案。答案数组中的元素应当按 **升序** 排列。

#### 示例 1:
![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/03/17/picture1.png)
<pre>
<strong>输入:</strong> graph = [[1,2],[2,3],[5],[0],[5],[],[]]
<strong>输出:</strong> [2,4,5,6]
<strong>解释:</strong> 示意图如上。
节点 5 和节点 6 是终端节点，因为它们都没有出边。
从节点 2、4、5 和 6 开始的所有路径都指向节点 5 或 6 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> graph = [[1,2,3,4],[1,2],[3,4],[0,4],[]]
<strong>输出:</strong> [4]
<strong>解释:</strong>
只有节点 4 是终端节点，从节点 4 开始的所有路径都通向节点 4 。
</pre>

#### 提示:
* `n == graph.length`
* <code>1 <= n <= 10<sup>4</sup></code>
* `0 <= graph[i].length <= n`
* `0 <= graph[i][j] <= n - 1`
* `graph[i]` 按严格递增顺序排列。
* 图中可能包含自环。
* 图中边的数目在范围 <code>[1, 4 * 10<sup>4</sup>]</code> 内。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut graph_rev = vec![vec![]; n];
        let mut outdegree = vec![0; n];
        let mut stack = vec![];
        let mut ret = vec![];

        for i in 0..n {
            for &j in &graph[i] {
                graph_rev[j as usize].push(i);
            }
            outdegree[i] = graph[i].len();
            if outdegree[i] == 0 {
                stack.push(i);
            }
        }

        while let Some(i) = stack.pop() {
            ret.push(i as i32);

            for &j in &graph_rev[i] {
                outdegree[j] -= 1;
                if outdegree[j] == 0 {
                    stack.push(j);
                }
            }
        }

        ret.sort_unstable();

        ret
    }
}
```
