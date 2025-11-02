# 1192. Critical Connections in a Network
There are `n` servers numbered from `0` to `n - 1` connected by undirected server-to-server `connections` forming a network where <code>connections[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> represents a connection between servers <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code>. Any server can reach other servers directly or indirectly through the network.

A *critical connection* is a connection that, if removed, will make some servers unable to reach some other server.

Return all critical connections in the network in any order.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/09/03/1537_ex1_2.png)
<pre>
<strong>Input:</strong> n = 4, connections = [[0,1],[1,2],[2,0],[1,3]]
<strong>Output:</strong> [[1,3]]
<strong>Explanation:</strong> [[3,1]] is also accepted.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 2, connections = [[0,1]]
<strong>Output:</strong> [[0,1]]
</pre>

#### Constraints:
* <code>2 <= n <= 10<sup>5</sup></code>
* <code>n - 1 <= connections.length <= 10<sup>5</sup></code>
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> <= n - 1</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* There are no repeated connections.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def criticalConnections(self, n: int, connections: List[List[int]]) -> List[List[int]]:
        def tarjan(a: int, parent: int) -> None:
            nonlocal time
            disc[a] = time
            low[a] = time
            time += 1

            for b in neighbors[a]:
                if b == parent:
                    continue

                if disc[b] == -1:
                    tarjan(b, a)
                    low[a] = min(low[a], low[b])

                    if low[b] > disc[a]:
                        ret.append([a, b])
                else:
                    low[a] = min(low[a], disc[b])

        neighbors = [[] for _ in range(n)]
        time = 0
        disc = [-1] * n
        low = [-1] * n
        ret = []

        for a, b in connections:
            neighbors[a].append(b)
            neighbors[b].append(a)

        tarjan(0, -1)

        return ret
```
