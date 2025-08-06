# 2493. Divide Nodes Into the Maximum Number of Groups
You are given a positive integer `n` representing the number of nodes in an **undirected** graph. The nodes are labeled from `1` to `n`.

You are also given a 2D integer array `edges`, where <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> indicates that there is a **bidirectional** edge between nodes <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code>. Notice that the given graph may be disconnected.

Divide the nodes of the graph into `m` groups (**1-indexed**) such that:
* Each node in the graph belongs to exactly one group.
* For every pair of nodes in the graph that are connected by an edge <code>[a<sub>i</sub>, b<sub>i</sub>]</code>, if <code>a<sub>i</sub></code> belongs to the group with index `x`, and <code>b<sub>i</sub></code> belongs to the group with index `y`, then `|y - x| = 1`.

Return *the maximum number of groups (i.e., maximum* `m`*) into which you can divide the nodes*. Return `-1` *if it is impossible to group the nodes with the given conditions*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/10/13/example1.png)
<pre>
<strong>Input:</strong> n = 6, edges = [[1,2],[1,4],[1,5],[2,6],[2,3],[4,6]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> As shown in the image we:
- Add node 5 to the first group.
- Add node 1 to the second group.
- Add nodes 2 and 4 to the third group.
- Add nodes 3 and 6 to the fourth group.
We can see that every edge is satisfied.
It can be shown that that if we create a fifth group and move any node from the third or fourth group to it, at least on of the edges will not be satisfied.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 3, edges = [[1,2],[2,3],[3,1]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> If we add node 1 to the first group, node 2 to the second group, and node 3 to the third group to satisfy the first two edges, we can see that the third edge will not be satisfied.
It can be shown that no grouping is possible.
</pre>

#### Constraints:
* `1 <= n <= 500`
* <code>1 <= edges.length <= 10<sup>4</sup></code>
* `edges[i].length == 2`
* <code>1 <= a<sub>i</sub>, b<sub>i</sub> <= n</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* There is at most one edge between any pair of vertices.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def magnificentSets(self, n: int, edges: List[List[int]]) -> int:
        parent = list(range(n + 1))
        neighbors = [[] for _ in range(n + 1)]
        unionfinds = {}
        ret = 0

        for a, b in edges:
            while parent[a] != parent[parent[a]]:
                parent[a] = parent[parent[a]]
            while parent[b] != parent[parent[b]]:
                parent[b] = parent[parent[b]]

            parent[parent[a]] = parent[b]
            neighbors[a].append(b)
            neighbors[b].append(a)

        for node in range(1, n + 1):
            while parent[node] != parent[parent[node]]:
                parent[node] = parent[parent[node]]

            if parent[node] not in unionfinds:
                unionfinds[parent[node]] = []
            unionfinds[parent[node]].append(node)

        for unionfind in unionfinds.values():
            m = 1

            for node in unionfind:
                used = {node: 1}
                queue = deque([node])

                while queue:
                    node = queue.popleft()
                    i = used[node]

                    for neighbor in neighbors[node]:
                        if neighbor not in used:
                            used[neighbor] = i + 1
                            queue.append(neighbor)
                            m = max(m, i + 1)
                        elif abs(used[neighbor] - i) != 1:
                            return -1

            ret += m

        return ret
```
