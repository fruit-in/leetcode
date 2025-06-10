# 310. 最小高度树
树是一个无向图，其中任何两个顶点只通过一条路径连接。 换句话说，任何一个没有简单环路的连通图都是一棵树。

给你一棵包含 `n` 个节点的树，标记为 `0` 到 `n - 1` 。给定数字 `n` 和一个有 `n - 1` 条无向边的 `edges` 列表（每一个边都是一对标签），其中 <code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>]</code> 表示树中节点 <code>a<sub>i</sub></code> 和 <code>b<sub>i</sub></code> 之间存在一条无向边。

可选择树中任何一个节点作为根。当选择节点 `x` 作为根节点时，设结果树的高度为 `h` 。在所有可能的树中，具有最小高度的树（即，`min(h)`）被称为 **最小高度树** 。

请你找到所有的 **最小高度树** 并按 **任意顺序** 返回它们的根节点标签列表。

树的 **高度** 是指根节点和叶子节点之间最长向下路径上边的数量。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/09/01/e1.jpg)
<pre>
<strong>输入:</strong> n = 4, edges = [[1,0],[1,2],[1,3]]
<strong>输出:</strong> [1]
<strong>解释:</strong> 如图所示，当根是标签为 1 的节点时，树的高度是 1 ，这是唯一的最小高度树。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2020/09/01/e2.jpg)
<pre>
<strong>输入:</strong> n = 6, edges = [[3,0],[3,1],[3,2],[3,4],[5,4]]
<strong>输出:</strong> [3,4]
</pre>

#### 提示:
* <code>1 <= n <= 2 * 10<sup>4</sup></code>
* `edges.length == n - 1`
* <code>0 <= a<sub>i</sub>, b<sub>i</sub> < n</code>
* <code>a<sub>i</sub> != b<sub>i</sub></code>
* 所有 <code>(a<sub>i</sub>, b<sub>i</sub>)</code> 互不相同
* 给定的输入 **保证** 是一棵树，并且 **不会有重复的边**

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def findMinHeightTrees(self, n: int, edges: List[List[int]]) -> List[int]:
        def removeParentFromChildren(node: int, parent: int) -> None:
            i = children[node].index(parent)
            children[node][i], children[node][-1] = children[node][-1], children[node][i]
            children[node].pop()
            for child in children[node]:
                removeParentFromChildren(child, node)

        def setHeight(node: int) -> None:
            for child in children[node]:
                setHeight(child)
                heights[node] = max(heights[node], heights[child] + 1)

        def findMinHeight(node: int, p: int) -> None:
            minheights[node] = max(heights[node], p)
            if len(children[node]) > 1:
                top2 = [children[node][0], children[node][1]]
                if heights[top2[0]] < heights[top2[1]]:
                    top2[0], top2[1] = top2[1], top2[0]
                for child in children[node][2:]:
                    if heights[child] >= heights[top2[0]]:
                        top2 = [child, top2[0]]
                    elif heights[child] >= heights[top2[1]]:
                        top2[1] = child
                for child in children[node]:
                    if child != top2[0]:
                        findMinHeight(child, max(p + 1, heights[top2[0]] + 2))
                    else:
                        findMinHeight(child, max(p + 1, heights[top2[1]] + 2))
            elif len(children[node]) == 1:
                findMinHeight(children[node][0], p + 1)

        children = [[] for _ in range(n)]
        children[0].append(-1)
        heights = [0] * n
        minheights = [0] * n

        for a, b in edges:
            children[a].append(b)
            children[b].append(a)

        removeParentFromChildren(0, -1)
        setHeight(0)
        findMinHeight(0, 0)
        minheight = min(minheights)

        return [node for node in range(n) if minheights[node] == minheight]
```
