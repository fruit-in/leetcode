# 1857. Largest Color Value in a Directed Graph
There is a **directed graph** of `n` colored nodes and `m` edges. The nodes are numbered from `0` to `n - 1`.

You are given a string `colors` where `colors[i]` is a lowercase English letter representing the color of the <code>i<sup>th</sup></code> node in this graph (**0-indexed**). You are also given a 2D array `edges` where <code>edges[j] = [a<sub>j</sub>, b<sub>j</sub>]</code> indicates that there is a **directed edge** from node <code>a<sub>j</sub></code> to node <code>b<sub>j</sub></code>.

A valid **path** in the graph is a sequence of nodes <code>x<sub>1</sub> -> x<sub>2</sub> -> x<sub>3</sub> -> ... -> x<sub>k</sub></code> such that there is a directed edge from <code>x<sub>i</sub></code> to <code>x<sub>i+1</sub></code> for every `1 <= i < k`. The **color value** of the path is the number of nodes that are colored the **most frequently** occurring color along that path.

Return *the **largest color value** of any valid path in the given graph, or* `-1` *if the graph contains a cycle*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/04/21/leet1.png)
<pre>
<strong>Input:</strong> colors = "abaca", edges = [[0,1],[0,2],[2,3],[3,4]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The path 0 -> 2 -> 3 -> 4 contains 3 nodes that are colored "a" (red in the above image).
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/04/21/leet2.png)
<pre>
<strong>Input:</strong> colors = "a", edges = [[0,0]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> There is a cycle from 0 to 0.
</pre>

#### Constraints:
* `n == colors.length`
* `m == edges.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>0 <= m <= 10<sup>5</sup></code>
* `colors` consists of lowercase English letters.
* <code>0 <= a<sub>j</sub>, b<sub>j</sub> < n</code>

## Solutions (Rust)

### 1. Solution
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
