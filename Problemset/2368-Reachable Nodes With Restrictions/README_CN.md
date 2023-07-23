# 2368. 受限条件下可到达节点的数目
现有一棵由 `n` 个节点组成的无向树，节点编号从 `0` 到 `n - 1` ，共有 `n - 1` 条边。

给你一个二维整数数组 `edges` ，长度为 `n - 1` ，其中 <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> 表示树中节点 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 之间存在一条边。另给你一个整数数组 `restricted` 表示 **受限** 节点。

在不访问受限节点的前提下，返回你可以从节点 `0` 到达的 **最多** 节点数目。

注意，节点 `0` **不** 会标记为受限节点。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/06/15/ex1drawio.png)
<pre>
<strong>输入:</strong> n = 7, edges = [[0,1],[1,2],[3,1],[4,0],[0,5],[5,6]], restricted = [4,5]
<strong>输出:</strong> 4
<strong>解释:</strong> 上图所示正是这棵树。
在不访问受限节点的前提下，只有节点 [0,1,2,3] 可以从节点 0 到达。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/06/15/ex2drawio.png)
<pre>
<strong>输入:</strong> n = 7, edges = [[0,1],[0,2],[0,5],[0,4],[3,2],[6,5]], restricted = [4,2,1]
<strong>输出:</strong> 3
<strong>解释:</strong> 上图所示正是这棵树。
在不访问受限节点的前提下，只有节点 [0,5,6] 可以从节点 0 到达。
</pre>

#### 提示:
* <code>2 <= n <= 10<sup>5</sup></code>
* `edges.length == n - 1`
* `edges[i].length == 2`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> < n</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* `edges` 表示一棵有效的树
* `1 <= restricted.length < n`
* `1 <= restricted[i] < n`
* `restricted` 中的所有值 **互不相同**

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def reachableNodes(self, n: int, edges: List[List[int]], restricted: List[int]) -> int:
        restricted = set(restricted)
        edgesmap = {}
        visited = {0}
        nodes = [0]

        for a, b in edges:
            if a not in edgesmap:
                edgesmap[a] = []
            if b not in edgesmap:
                edgesmap[b] = []
            edgesmap[a].append(b)
            edgesmap[b].append(a)

        while nodes:
            curr = nodes.pop()
            for node in edgesmap[curr]:
                if node not in restricted and node not in visited:
                    visited.add(node)
                    nodes.append(node)

        return len(visited)
```
