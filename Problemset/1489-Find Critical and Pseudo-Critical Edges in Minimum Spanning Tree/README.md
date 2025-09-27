# 1489. Find Critical and Pseudo-Critical Edges in Minimum Spanning Tree
Given a weighted undirected connected graph with `n` vertices numbered from `0` to `n - 1`, and an array `edges` where <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>, weight<sub>i</sub>]</code> represents a bidirectional and weighted edge between nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code>. A minimum spanning tree (MST) is a subset of the graph's edges that connects all vertices without cycles and with the minimum possible total edge weight.

Find *all the critical and pseudo-critical edges in the given graph's minimum spanning tree (MST)*. An MST edge whose deletion from the graph would cause the MST weight to increase is called a *critical edge*. On the other hand, a pseudo-critical edge is that which can appear in some MSTs but not all.

Note that you can return the indices of the edges in any order.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/06/04/ex1.png)
<pre>
<strong>Input:</strong> n = 5, edges = [[0,1,1],[1,2,1],[2,3,2],[0,3,2],[0,4,3],[3,4,3],[1,4,6]]
<strong>Output:</strong> [[0,1],[2,3,4,5]]
<strong>Explanation:</strong> The figure above describes the graph.
The following figure shows all the possible MSTs:
<img src="https://assets.leetcode.com/uploads/2020/06/04/msts.png">
Notice that the two edges 0 and 1 appear in all MSTs, therefore they are critical edges, so we return them in the first list of the output.
The edges 2, 3, 4, and 5 are only part of some MSTs, therefore they are considered pseudo-critical edges. We add them to the second list of the output.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/06/04/ex2.png)
<pre>
<strong>Input:</strong> n = 4, edges = [[0,1,1],[1,2,1],[2,3,1],[0,3,1]]
<strong>Output:</strong> [[],[0,1,2,3]]
<strong>Explanation:</strong> We can observe that since all 4 edges have equal weight, choosing any 3 edges from the given 4 will yield an MST. Therefore all 4 edges are pseudo-critical.
</pre>

#### Constraints:
* `2 <= n <= 100`
* `1 <= edges.length <= min(200, n * (n - 1) / 2)`
* `edges[i].length == 3`
* <code>0 <= a<sub>i</sub> < b<sub>i</sub> < n</code>
* <code>1 <= weight<sub>i</sub> <= 1000</code>
* All pairs <code>(a<sub>i</sub>, b<sub>i</sub>)</code> are **distinct**.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def findCriticalAndPseudoCriticalEdges(self, n: int, edges: List[List[int]]) -> List[List[int]]:
        def getMSTWeight(initial: (int, int, int, int), delete: int = -1) -> int:
            if initial[0] == delete:
                initial = edges[1]

            parent = list(range(n))
            parent[initial[1]] = initial[2]
            total = initial[3]
            edgecount = 1

            for i, a, b, w in edges:
                if i == initial[0] or i == delete:
                    continue

                while parent[a] != parent[parent[a]]:
                    parent[a] = parent[parent[a]]
                while parent[b] != parent[parent[b]]:
                    parent[b] = parent[parent[b]]

                if parent[a] != parent[b]:
                    parent[parent[a]] = parent[b]
                    total += w
                    edgecount += 1

            return total if edgecount == n - 1 else inf

        if n == 2:
            return [[0], []]

        edges = [(i, a, b, w) for i, [a, b, w] in enumerate(edges)]
        edges.sort(key=lambda edge: edge[3])
        minweight = getMSTWeight(edges[0])
        ret = [[], []]

        for edge in edges:
            if getMSTWeight(edges[0], edge[0]) > minweight:
                ret[0].append(edge[0])
            elif getMSTWeight(edge) == minweight:
                ret[1].append(edge[0])

        return ret
```
