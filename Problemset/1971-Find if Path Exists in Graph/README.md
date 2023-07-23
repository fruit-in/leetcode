# 1971. Find if Path Exists in Graph
There is a **bi-directional** graph with `n` vertices, where each vertex is labeled from `0` to `n - 1` (**inclusive**). The edges in the graph are represented as a 2D integer array `edges`, where each <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> denotes a bi-directional edge between vertex <code>u<sub>i</sub></code> and vertex <code>v<sub>i</sub></code>. Every vertex pair is connected by **at most one** edge, and no vertex has an edge to itself.

You want to determine if there is a **valid path** that exists from vertex `source` to vertex `destination`.

Given `edges` and the integers `n`, `source`, and `destination`, return `true` *if there is a **valid path** from* `source` *to* `destination`, *or* `false` *otherwise*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/08/14/validpath-ex1.png)
<pre>
<strong>Input:</strong> n = 3, edges = [[0,1],[1,2],[2,0]], source = 0, destination = 2
<strong>Output:</strong> true
<strong>Explanation:</strong> There are two paths from vertex 0 to vertex 2:
- 0 → 1 → 2
- 0 → 2
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/08/14/validpath-ex2.png)
<pre>
<strong>Input:</strong> n = 6, edges = [[0,1],[0,2],[3,5],[5,4],[4,3]], source = 0, destination = 5
<strong>Output:</strong> false
<strong>Explanation:</strong> There is no path from vertex 0 to vertex 5.
</pre>

#### Constraints:
* <code>1 <= n <= 2 * 10<sup>5</sup></code>
* <code>0 <= edges.length <= 2 * 10<sup>5</sup></code>
* `edges[i].length == 2`
* <code>0 <= u<sub>i</sub>, v<sub>i</sub> <= n - 1</code>
* <code>u<sub>i</sub> != v<sub>i</sub></code>
* `0 <= source, destination <= n - 1`
* There are no duplicate edges.
* There are no self edges.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def validPath(self, n: int, edges: List[List[int]], source: int, destination: int) -> bool:
        paths = {}
        nodes = [source]
        seen = {source}

        for u, v in edges:
            if u not in paths:
                paths[u] = []
            if v not in paths:
                paths[v] = []
            paths[u].append(v)
            paths[v].append(u)

        while nodes:
            node0 = nodes.pop()
            for node1 in paths.get(node0, [node0]):
                if node1 == destination:
                    return True

                if node1 not in seen:
                    nodes.append(node1)
                    seen.add(node1)

        return False
```
