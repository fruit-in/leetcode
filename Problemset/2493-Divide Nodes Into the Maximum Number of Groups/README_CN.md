# 2493. 将节点分成尽可能多的组
给你一个正整数 `n` ，表示一个 **无向** 图中的节点数目，节点编号从 `1` 到 `n` 。

同时给你一个二维整数数组 `edges` ，其中 <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> 表示节点 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 之间有一条 **双向** 边。注意给定的图可能是不连通的。

请你将图划分为 `m` 个组（编号从 **1** 开始），满足以下要求：
* 图中每个节点都只属于一个组。
* 图中每条边连接的两个点 <code>[a<sub>i</sub>, b<sub>i</sub>]</code> ，如果 <code>a<sub>i</sub></code> 属于编号为 `x` 的组，<code>b<sub>i</sub></code> 属于编号为 `y` 的组，那么 `|y - x| = 1` 。

请你返回最多可以将节点分为多少个组（也就是最大的 `m` ）。如果没办法在给定条件下分组，请你返回 `-1` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/10/13/example1.png)
<pre>
<strong>输入:</strong> n = 6, edges = [[1,2],[1,4],[1,5],[2,6],[2,3],[4,6]]
<strong>输出:</strong> 4
<strong>解释:</strong> 如上图所示，
- 节点 5 在第一个组。
- 节点 1 在第二个组。
- 节点 2 和节点 4 在第三个组。
- 节点 3 和节点 6 在第四个组。
所有边都满足题目要求。
如果我们创建第五个组，将第三个组或者第四个组中任何一个节点放到第五个组，至少有一条边连接的两个节点所属的组编号不符合题目要求。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 3, edges = [[1,2],[2,3],[3,1]]
<strong>输出:</strong> -1
<strong>解释:</strong> 如果我们将节点 1 放入第一个组，节点 2 放入第二个组，节点 3 放入第三个组，前两条边满足题目要求，但第三条边不满足题目要求。
没有任何符合题目要求的分组方式。
</pre>

#### 提示:
* `1 <= n <= 500`
* <code>1 <= edges.length <= 10<sup>4</sup></code>
* `edges[i].length == 2`
* <code>1 <= a<sub>i</sub>, b<sub>i</sub> <= n</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* 两个点之间至多只有一条边。

## 题解 (Python)

### 1. 题解
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
