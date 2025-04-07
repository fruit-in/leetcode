# 1857. 有向图中最大颜色值
给你一个 **有向图** ，它含有 `n` 个节点和 `m` 条边。节点编号从 `0` 到 `n - 1` 。

给你一个字符串 `colors` ，其中 `colors[i]` 是小写英文字母，表示图中第 `i` 个节点的 **颜色** （下标从 **0** 开始）。同时给你一个二维数组 `edges` ，其中 <code>edges[j] = [a<sub>j</sub>, b<sub>j</sub>]</code> 表示从节点 <code>a<sub>j</sub></code> 到节点 <code>b<sub>j</sub></code> 有一条 **有向边** 。

图中一条有效 **路径** 是一个点序列 <code>x<sub>1</sub> -> x<sub>2</sub> -> x<sub>3</sub> -> ... -> x<sub>k</sub></code> ，对于所有 `1 <= i < k` ，从 <code>x<sub>i</sub></code> 到 <code>x<sub>i+1</sub></code> 在图中有一条有向边。路径的 **颜色值** 是路径中 **出现次数最多** 颜色的节点数目。

请你返回给定图中有效路径里面的 **最大颜色值** 。如果图中含有环，请返回 `-1` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/04/21/leet1.png)
<pre>
<strong>输入:</strong> colors = "abaca", edges = [[0,1],[0,2],[2,3],[3,4]]
<strong>输出:</strong> 3
<strong>解释:</strong> 路径 0 -> 2 -> 3 -> 4 含有 3 个颜色为 "a" 的节点（上图中的红色节点）。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/04/21/leet2.png)
<pre>
<strong>输入:</strong> colors = "a", edges = [[0,0]]
<strong>输出:</strong> -1
<strong>解释:</strong> 从 0 到 0 有一个环。
</pre>

#### 提示:
* `n == colors.length`
* `m == edges.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>0 <= m <= 10<sup>5</sup></code>
* `colors` 只含有小写英文字母。
* <code>0 <= a<sub>j</sub>, b<sub>j</sub> < n</code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let colors = colors
            .bytes()
            .map(|c| (c - b'a') as usize)
            .collect::<Vec<_>>();
        let n = colors.len();
        let mut indegree = vec![0; n];
        let mut next_nodes = vec![vec![]; n];
        let mut nodes = vec![];
        let mut node_colors = vec![[0; 26]; n];
        let mut count = 0;

        for edge in &edges {
            indegree[edge[1] as usize] += 1;
            next_nodes[edge[0] as usize].push(edge[1] as usize);
        }

        for i in 0..n {
            if indegree[i] == 0 {
                nodes.push(i);
            }
            node_colors[i][colors[i]] += 1;
        }

        while let Some(i) = nodes.pop() {
            count += 1;

            for &j in &next_nodes[i] {
                indegree[j] -= 1;
                if indegree[j] == 0 {
                    nodes.push(j);
                }

                for k in 0..26 {
                    if k != colors[j] {
                        node_colors[j][k] = node_colors[j][k].max(node_colors[i][k]);
                    } else {
                        node_colors[j][k] = node_colors[j][k].max(node_colors[i][k] + 1);
                    }
                }
            }
        }

        if count < n {
            return -1;
        }

        (0..n).map(|i| node_colors[i][colors[i]]).max().unwrap()
    }
}
```
