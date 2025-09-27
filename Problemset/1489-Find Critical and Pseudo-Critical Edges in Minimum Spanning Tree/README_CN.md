# 1489. 找到最小生成树里的关键边和伪关键边
给你一个 `n` 个点的带权无向连通图，节点编号为 `0` 到 `n-1` ，同时还有一个数组 `edges` ，其中 <code>edges[i] = [from<sub>i</sub>, to<sub>i</sub>, weight<sub>i</sub>]</code> 表示在 <code>from<sub>i</sub></code> 和 <code>to<sub>i</sub></code> 节点之间有一条带权无向边。最小生成树 (MST) 是给定图中边的一个子集，它连接了所有节点且没有环，而且这些边的权值和最小。

请你找到给定图中最小生成树的所有关键边和伪关键边。如果从图中删去某条边，会导致最小生成树的权值和增加，那么我们就说它是一条关键边。伪关键边则是可能会出现在某些最小生成树中但不会出现在所有最小生成树中的边。

请注意，你可以分别以任意顺序返回关键边的下标和伪关键边的下标。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/06/04/ex1.png)
<pre>
<strong>输入:</strong> n = 5, edges = [[0,1,1],[1,2,1],[2,3,2],[0,3,2],[0,4,3],[3,4,3],[1,4,6]]
<strong>输出:</strong> [[0,1],[2,3,4,5]]
<strong>解释:</strong> 上图描述了给定图。
下图是所有的最小生成树。
<img src="https://assets.leetcode.com/uploads/2020/06/04/msts.png">
注意到第 0 条边和第 1 条边出现在了所有最小生成树中，所以它们是关键边，我们将这两个下标作为输出的第一个列表。
边 2，3，4 和 5 是所有 MST 的剩余边，所以它们是伪关键边。我们将它们作为输出的第二个列表。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/06/04/ex2.png)
<pre>
<strong>输入:</strong> n = 4, edges = [[0,1,1],[1,2,1],[2,3,1],[0,3,1]]
<strong>输出:</strong> [[],[0,1,2,3]]
<strong>解释:</strong> 可以观察到 4 条边都有相同的权值，任选它们中的 3 条可以形成一棵 MST 。所以 4 条边都是伪关键边。
</pre>

#### 提示:
* `2 <= n <= 100`
* `1 <= edges.length <= min(200, n * (n - 1) / 2)`
* `edges[i].length == 3`
* <code>0 <= a<sub>i</sub> < b<sub>i</sub> < n</code>
* <code>1 <= weight<sub>i</sub> <= 1000</code>
* 所有 <code>(from<sub>i</sub>, to<sub>i</sub>)</code> 数对都是互不相同的。

## 题解 (Python)

### 1. 题解
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
