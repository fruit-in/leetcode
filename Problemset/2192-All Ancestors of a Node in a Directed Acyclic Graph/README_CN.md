# 2192. 有向无环图中一个节点的所有祖先
给你一个正整数 `n` ，它表示一个 **有向无环图** 中节点的数目，节点编号为 `0` 到 `n - 1` （包括两者）。

给你一个二维整数数组 `edges` ，其中 <code>edges[i] = [from<sub>i</sub>, to<sub>i</sub>]</code> 表示图中一条从 <code>from<sub>i</sub></code> 到 <code>to<sub>i</sub></code> 的单向边。

请你返回一个数组 `answer`，其中 `answer[i]`是第 `i` 个节点的所有 **祖先** ，这些祖先节点 **升序** 排序。

如果 `u` 通过一系列边，能够到达 `v` ，那么我们称节点 `u` 是节点 `v` 的 **祖先** 节点。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2019/12/12/e1.png)
<pre>
<strong>输入:</strong> n = 8, edgeList = [[0,3],[0,4],[1,3],[2,4],[2,7],[3,5],[3,6],[3,7],[4,6]]
<strong>输出:</strong> [[],[],[],[0,1],[0,2],[0,1,3],[0,1,2,3,4],[0,1,2,3]]
<strong>解释:</strong>
上图为输入所对应的图。
- 节点 0 ，1 和 2 没有任何祖先。
- 节点 3 有 2 个祖先 0 和 1 。
- 节点 4 有 2 个祖先 0 和 2 。
- 节点 5 有 3 个祖先 0 ，1 和 3 。
- 节点 6 有 5 个祖先 0 ，1 ，2 ，3 和 4 。
- 节点 7 有 4 个祖先 0 ，1 ，2 和 3 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2019/12/12/e2.png)
<pre>
<strong>输入:</strong> n = 5, edgeList = [[0,1],[0,2],[0,3],[0,4],[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]
<strong>输出:</strong> [[],[0],[0,1],[0,1,2],[0,1,2,3]]
<strong>解释:</strong>
上图为输入所对应的图。
- 节点 0 没有任何祖先。
- 节点 1 有 1 个祖先 0 。
- 节点 2 有 2 个祖先 0 和 1 。
- 节点 3 有 3 个祖先 0 ，1 和 2 。
- 节点 4 有 4 个祖先 0 ，1 ，2 和 3 。
</pre>

#### 提示:
* `1 <= n <= 1000`
* `0 <= edges.length <= min(2000, n * (n - 1) / 2)`
* `edges[i].length == 2`
* <code>0 <= from<sub>i</sub>, to<sub>i</sub> <= n - 1</code>
* <code>from<sub>i</sub> != to<sub>i</sub></code>
* 图中不会有重边。
* 图是 **有向** 且 **无环** 的。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def getAncestors(self, n: int, edges: List[List[int]]) -> List[List[int]]:
        indegrees = [0] * n
        ancestors = [set() for _ in range(n)]
        children = [[] for _ in range(n)]
        nodes = []
        answer = [[] for _ in range(n)]

        for u, v in edges:
            indegrees[v] += 1
            children[u].append(v)

        for i in range(n):
            if indegrees[i] == 0:
                nodes.append(i)

        while nodes != []:
            i = nodes.pop()
            answer[i] = sorted(ancestors[i])

            for j in children[i]:
                indegrees[j] -= 1
                ancestors[j].add(i)
                ancestors[j] |= ancestors[i]
                if indegrees[j] == 0:
                    nodes.append(j)

        return answer
```
