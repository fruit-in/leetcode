# 1319. 连通网络的操作次数
用以太网线缆将 `n` 台计算机连接成一个网络，计算机的编号从 `0` 到 `n-1`。线缆用 `connections` 表示，其中 `connections[i] = [a, b]` 连接了计算机 `a` 和 `b`。

网络中的任何一台计算机都可以通过网络直接或者间接访问同一个网络中其他任意一台计算机。

给你这个计算机网络的初始布线 `connections`，你可以拔开任意两台直连计算机之间的线缆，并用它连接一对未直连的计算机。请你计算并返回使所有计算机都连通所需的最少操作次数。如果不可能，则返回 -1 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/01/02/sample_1_1677.png)
<pre>
<strong>输入:</strong> n = 4, connections = [[0,1],[0,2],[1,2]]
<strong>输出:</strong> 1
<strong>解释:</strong> 拔下计算机 1 和 2 之间的线缆，并将它插到计算机 1 和 3 上。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/01/02/sample_2_1677.png)
<pre>
<strong>输入:</strong> n = 6, connections = [[0,1],[0,2],[0,3],[1,2],[1,3]]
<strong>输出:</strong> 2
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 6, connections = [[0,1],[0,2],[0,3],[1,2]]
<strong>输出:</strong> -1
<strong>解释:</strong> 线缆数量不足。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> n = 5, connections = [[0,1],[0,2],[3,4],[2,3]]
<strong>输出:</strong> 0
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= connections.length <= min(n * (n - 1) / 2, 10<sup>5</sup>)</code>
* `connections[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> < n</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* 没有重复的连接。
* 两台计算机不会通过多条线缆连接。

## 题解 (Python)

### 1. 题解
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
