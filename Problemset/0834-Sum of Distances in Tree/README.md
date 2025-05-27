# 834. Sum of Distances in Tree
There is an undirected connected tree with `n` nodes labeled from `0` to `n - 1` and `n - 1` edges.

You are given the integer `n` and the array `edges` where <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that there is an edge between nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code> in the tree.

Return an array `answer` of length `n` where `answer[i]` is the sum of the distances between the <code>i<sup>th</sup></code> node in the tree and all other nodes.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/07/23/lc-sumdist1.jpg)
<pre>
<strong>Input:</strong> n = 6, edges = [[0,1],[0,2],[2,3],[2,4],[2,5]]
<strong>Output:</strong> [8,12,6,10,10,10]
<strong>Explanation:</strong> The tree is shown above.
We can see that dist(0,1) + dist(0,2) + dist(0,3) + dist(0,4) + dist(0,5)
equals 1 + 1 + 2 + 2 + 2 = 8.
Hence, answer[0] = 8, and so on.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/07/23/lc-sumdist2.jpg)
<pre>
<strong>Input:</strong> n = 1, edges = []
<strong>Output:</strong> [0]
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/07/23/lc-sumdist3.jpg)
<pre>
<strong>Input:</strong> n = 2, edges = [[1,0]]
<strong>Output:</strong> [1,1]
</pre>

#### Constraints:
* <code>1 <= n <= 3 * 10<sup>4</sup></code>
* `edges.length == n - 1`
* `edges[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> < n</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* The given input represents a valid tree.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut children = vec![vec![]; n];
        let mut parent = vec![n; n];
        let mut stack = vec![0];
        let mut indegree = vec![0; n];
        let mut subtree_nodes_count = vec![1; n];
        let mut answer = vec![0; n];

        for edge in &edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            children[a].push(b);
            children[b].push(a);
            indegree[a] += 1;
            indegree[b] += 1;
        }

        while let Some(i) = stack.pop() {
            for &j in &children[i] {
                if j != 0 && parent[j] == n {
                    parent[j] = i;
                    stack.push(j);
                }
            }
        }

        for i in 1..n {
            indegree[i] -= 1;
            if children[i].len() == 1 {
                children[i].pop();
                stack.push(i);
                continue;
            }

            for j in 0..children[i].len() {
                if children[i][j] == parent[i] {
                    children[i].swap_remove(j);
                    break;
                }
            }
        }

        while let Some(i) = stack.pop() {
            if i == 0 {
                break;
            }

            indegree[parent[i]] -= 1;
            if indegree[parent[i]] == 0 {
                stack.push(parent[i]);
            }
            subtree_nodes_count[parent[i]] += subtree_nodes_count[i];
            answer[parent[i]] += answer[i] + subtree_nodes_count[i];
        }

        stack.push(0);
        while let Some(i) = stack.pop() {
            for &j in &children[i] {
                answer[j] = answer[i] - subtree_nodes_count[j] * 2 + n as i32;
                stack.push(j);
            }
        }

        answer
    }
}
```
