# 2316. Count Unreachable Pairs of Nodes in an Undirected Graph
You are given an integer `n`. There is an **undirected** graph with `n` nodes, numbered from `0` to `n - 1`. You are given a 2D integer array `edges` where <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> denotes that there exists an **undirected** edge connecting nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code>.

Return *the **number of pairs** of different nodes that are **unreachable** from each other*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/05/05/tc-3.png)
<pre>
<strong>Input:</strong> n = 3, edges = [[0,1],[0,2],[1,2]]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There are no pairs of nodes that are unreachable from each other. Therefore, we return 0.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/05/05/tc-2.png)
<pre>
<strong>Input:</strong> n = 7, edges = [[0,2],[0,5],[2,4],[1,6],[5,4]]
<strong>Output:</strong> 14
<strong>Explanation:</strong> There are 14 pairs of nodes that are unreachable from each other:
[[0,1],[0,3],[0,6],[1,2],[1,3],[1,4],[1,5],[2,3],[2,6],[3,4],[3,5],[3,6],[4,6],[5,6]].
Therefore, we return 14.
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>0 <= edges.length <= 2 * 10<sup>5</sup></code>
* `edges[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> < n</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* There are no repeated edges.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def countPairs(self, n: int, edges: List[List[int]]) -> int:
        parent = list(range(n))
        count = [0] * n

        for a, b in edges:
            while parent[a] != parent[parent[a]]:
                parent[a] = parent[parent[a]]
            while parent[b] != parent[parent[b]]:
                parent[b] = parent[parent[b]]
            if parent[a] < parent[b]:
                parent[parent[b]] = parent[a]
            else:
                parent[parent[a]] = parent[b]

        for i in range(n):
            while parent[i] != parent[parent[i]]:
                parent[i] = parent[parent[i]]
            count[parent[i]] += 1

        return sum(c * (n - c) for c in count) // 2
```
