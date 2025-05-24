# 1319. Number of Operations to Make Network Connected
There are `n` computers numbered from `0` to `n - 1` connected by ethernet cables `connections` forming a network where <code>connections[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> represents a connection between computers <code>a<sub>i</sub></code> and <code>b<sub>i</sub></code>. Any computer can reach any other computer directly or indirectly through the network.

You are given an initial computer network `connections`. You can extract certain cables between two directly connected computers, and place them between any pair of disconnected computers to make them directly connected.

Return *the minimum number of times you need to do this in order to make all the computers connected*. If it is not possible, return `-1`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/01/02/sample_1_1677.png)
<pre>
<strong>Input:</strong> n = 4, connections = [[0,1],[0,2],[1,2]]
<strong>Output:</strong> 1
<strong>Explanation:</strong> Remove cable between computer 1 and 2 and place between computers 1 and 3.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/01/02/sample_2_1677.png)
<pre>
<strong>Input:</strong> n = 6, connections = [[0,1],[0,2],[0,3],[1,2],[1,3]]
<strong>Output:</strong> 2
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 6, connections = [[0,1],[0,2],[0,3],[1,2]]
<strong>Output:</strong> -1
<strong>Explanation:</strong> There are not enough cables.
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= connections.length <= min(n * (n - 1) / 2, 10<sup>5</sup>)</code>
* `connections[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> < n</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* There are no repeated connections.
* No two computers are connected by more than one cable.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def makeConnected(self, n: int, connections: List[List[int]]) -> int:
        parent = list(range(n))
        groupnodes = {i: 1 for i in range(n)}
        groupcables = {i: 0 for i in range(n)}

        for a, b in connections:
            while parent[a] != parent[parent[a]]:
                parent[a] = parent[parent[a]]
            while parent[b] != parent[parent[b]]:
                parent[b] = parent[parent[b]]

            if parent[a] != parent[b]:
                groupnodes[parent[a]] += groupnodes.pop(parent[b])
                groupcables[parent[a]] += groupcables.pop(parent[b])
                parent[parent[b]] = parent[a]
            groupcables[parent[a]] += 1

        redundant = sum(groupcables[i] - groupnodes[i] + 1 for i in groupnodes)
        needed = len(groupnodes) - 1

        if redundant < needed:
            return -1

        return needed
```
