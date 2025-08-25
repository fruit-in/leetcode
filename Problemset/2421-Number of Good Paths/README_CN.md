# 2421. 好路径的数目
给你一棵 `n` 个节点的树（连通无向无环的图），节点编号从 `0` 到 `n - 1` 且恰好有 `n - 1` 条边。

给你一个长度为 `n` 下标从 **0** 开始的整数数组 `vals` ，分别表示每个节点的值。同时给你一个二维整数数组 `edges` ，其中 <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> 表示节点 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 之间有一条 **无向** 边。

一条 **好路径** 需要满足以下条件：
1. 开始节点和结束节点的值 **相同** 。
2. 开始节点和结束节点中间的所有节点值都 **小于等于** 开始节点的值（也就是说开始节点的值应该是路径上所有节点的最大值）。

请你返回不同好路径的数目。

注意，一条路径和它反向的路径算作 **同一** 路径。比方说， `0 -> 1` 与 `1 -> 0` 视为同一条路径。单个节点也视为一条合法路径。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/08/04/f9caaac15b383af9115c5586779dec5.png)
<pre>
<strong>输入:</strong> vals = [1,3,2,1,3], edges = [[0,1],[0,2],[2,3],[2,4]]
<strong>输出:</strong> 6
<strong>解释:</strong> 总共有 5 条单个节点的好路径。
还有 1 条好路径：1 -> 0 -> 2 -> 4 。
（反方向的路径 4 -> 2 -> 0 -> 1 视为跟 1 -> 0 -> 2 -> 4 一样的路径）
注意 0 -> 2 -> 3 不是一条好路径，因为 vals[2] > vals[0] 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/08/04/149d3065ec165a71a1b9aec890776ff.png)
<pre>
<strong>输入:</strong> vals = [1,1,2,2,3], edges = [[0,1],[1,2],[2,3],[2,4]]
<strong>输出:</strong> 7
<strong>解释:</strong> 总共有 5 条单个节点的好路径。
还有 2 条好路径：0 -> 1 和 2 -> 3 。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2022/08/04/31705e22af3d9c0a557459bc7d1b62d.png)
<pre>
<strong>输入:</strong> vals = [1], edges = []
<strong>输出:</strong> 1
<strong>解释:</strong> 这棵树只有一个节点，所以只有一条好路径。
</pre>

#### 提示:
* `n == vals.length`
* <code>1 <= n <= 3 * 10<sup>4</sup></code>
* <code>0 <= vals[i] <= 10<sup>5</sup></code>
* `edges.length == n - 1`
* `edges[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> < n</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* `edges` 表示一棵合法的树。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def numberOfGoodPaths(self, vals: List[int], edges: List[List[int]]) -> int:
        n = len(vals)
        parent = list(range(n))
        neighbors = [[] for _ in range(n)]
        valscount = Counter(vals)
        ret = sum(1 for c in valscount.values() if c == 1)

        for a, b in edges:
            neighbors[a].append(b)
            neighbors[b].append(a)

        for i in sorted(range(n), key=lambda i: vals[i]):
            if parent[i] != i or valscount[vals[i]] == 1:
                continue

            sameval = 1
            visited = {i}
            greater = set()

            while neighbors[i]:
                j = neighbors[i].pop()
                while parent[j] != parent[parent[j]]:
                    parent[j] = parent[parent[j]]
                j = parent[j]

                if vals[j] == vals[i]:
                    sameval += 1
                if vals[j] <= vals[i] and parent[j] != i:
                    parent[j] = i
                    for k in neighbors[j]:
                        while parent[k] != parent[parent[k]]:
                            parent[k] = parent[parent[k]]
                        k = parent[k]
                        if k not in visited:
                            neighbors[i].append(k)
                            visited.add(k)
                elif vals[j] > vals[i]:
                    greater.add(j)

            neighbors[i] = list(greater)
            ret += sameval * (sameval + 1) // 2

        return ret
```
