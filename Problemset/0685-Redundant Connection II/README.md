# 685. Redundant Connection II
In this problem, a rooted tree is a **directed** graph such that, there is exactly one node (the root) for which all other nodes are descendants of this node, plus every node has exactly one parent, except for the root node which has no parents.

The given input is a directed graph that started as a rooted tree with `n` nodes (with distinct values from `1` to `n`), with one additional directed edge added. The added edge has two different vertices chosen from `1` to `n`, and was not an edge that already existed.

The resulting graph is given as a 2D-array of `edges`. Each element of `edges` is a pair <code>[u<sub>i</sub>, v<sub>i</sub>]</code> that represents a **directed** edge connecting nodes <code>u<sub>i</sub></code> and <code>v<sub>i</sub></code>, where <code>u<sub>i</sub></code> is a parent of child <code>v<sub>i</sub></code>.

Return *an edge that can be removed so that the resulting graph is a rooted tree of* `n` *nodes*. If there are multiple answers, return the answer that occurs last in the given 2D-array.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/12/20/graph1.jpg)
<pre>
<strong>Input:</strong> edges = [[1,2],[1,3],[2,3]]
<strong>Output:</strong> [2,3]
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/12/20/graph2.jpg)
<pre>
<strong>Input:</strong> edges = [[1,2],[2,3],[3,4],[4,1],[1,5]]
<strong>Output:</strong> [4,1]
</pre>

#### Constraints:
* `n == edges.length`
* `3 <= n <= 1000`
* `edges[i].length == 2`
* <code>1 <= u<sub>i</sub>, v<sub>i</sub> <= n</code>
* <code>u<sub>i</sub> != v<sub>i</sub></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();

        for i in (0..n).rev() {
            let mut indegree = vec![0; n];
            let mut children = vec![vec![]; n];
            let mut stack = vec![];
            let mut count = 1;

            for j in 0..n {
                if j != i {
                    let (u, v) = (edges[j][0] as usize - 1, edges[j][1] as usize - 1);
                    indegree[v] += 1;
                    children[u].push(v);

                    if indegree[v] > 1 {
                        indegree[0] = 0;
                        indegree[1] = 0;
                        break;
                    }
                }
            }

            for j in 0..n {
                if indegree[j] == 0 {
                    stack.push(j);
                }
                if stack.len() > 1 {
                    stack.clear();
                    break;
                }
            }

            while let Some(j) = stack.pop() {
                for &k in &children[j] {
                    indegree[k] -= 1;
                    if indegree[k] == 0 {
                        stack.push(k);
                        count += 1;
                    }
                }
            }

            if count == n {
                return edges[i].clone();
            }
        }

        unreachable!()
    }
}
```
