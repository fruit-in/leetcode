# 1971. 寻找图中是否存在路径
有一个具有 `n` 个顶点的 双向 图，其中每个顶点标记从 `0` 到 `n - 1`（包含 `0` 和 `n - 1`）。图中的边用一个二维整数数组 `edges` 表示，其中 <code>edges[i] = [u<sub>i</sub>, v<sub>i</sub>]</code> 表示顶点 <code>u<sub>i</sub></code> 和顶点 <code>v<sub>i</sub></code> 之间的双向边。 每个顶点对由 **最多一条** 边连接，并且没有顶点存在与自身相连的边。

请你确定是否存在从顶点 `start` 开始，到顶点 `end` 结束的 **有效路径** 。

给你数组 `edges` 和整数 `n`、`start`和`end`，如果从 `start` 到 `end` 存在 **有效路径** ，则返回 `true`，否则返回 `false` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/08/14/validpath-ex1.png)
<pre>
<strong>输入:</strong> n = 3, edges = [[0,1],[1,2],[2,0]], source = 0, destination = 2
<strong>输出:</strong> true
<strong>解释:</strong> 存在由顶点 0 到顶点 2 的路径:
- 0 → 1 → 2
- 0 → 2
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/08/14/validpath-ex2.png)
<pre>
<strong>输入:</strong> n = 6, edges = [[0,1],[0,2],[3,5],[5,4],[4,3]], source = 0, destination = 5
<strong>输出:</strong> false
<strong>解释:</strong> 不存在由顶点 0 到顶点 5 的路径.
</pre>

#### 提示:
* <code>1 <= n <= 2 * 10<sup>5</sup></code>
* <code>0 <= edges.length <= 2 * 10<sup>5</sup></code>
* `edges[i].length == 2`
* <code>0 <= u<sub>i</sub>, v<sub>i</sub> <= n - 1</code>
* <code>u<sub>i</sub> != v<sub>i</sub></code>
* `0 <= source, destination <= n - 1`
* 不存在双向边
* 不存在指向顶点自身的边

## 题解 (Python)

### 1. 题解
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
