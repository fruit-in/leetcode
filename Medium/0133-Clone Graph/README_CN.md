# 133. 克隆图
给你无向 **[连通](https://baike.baidu.com/item/%E8%BF%9E%E9%80%9A%E5%9B%BE/6460995?fr=aladdin)** 图中一个节点的引用，请你返回该图的 **[深拷贝](https://baike.baidu.com/item/%E6%B7%B1%E6%8B%B7%E8%B4%9D/22785317?fr=aladdin)**（克隆）。

图中的每个节点都包含它的值 `val`（`int`） 和其邻居的列表（`list[Node]`）。

```
class Node {
    public int val;
    public List<Node> neighbors;
}
```

#### 测试用例格式:
简单起见，每个节点的值都和它的索引相同。例如，第一个节点值为 1（`val = 1`），第二个节点值为 2（`val = 2`），以此类推。该图在测试用例中使用邻接列表表示。

**邻接列表** 是用于表示有限图的无序列表的集合。每个列表都描述了图中节点的邻居集。

给定节点将始终是图中的第一个节点（值为 1）。你必须将 **给定节点的拷贝** 作为对克隆图的引用返回。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/02/01/133_clone_graph_question.png)
<pre>
<strong>输入:</strong> adjList = [[2,4],[1,3],[2,4],[1,3]]
<strong>输出:</strong> [[2,4],[1,3],[2,4],[1,3]]
<strong>解释:</strong>
图中有 4 个节点。
节点 1 的值是 1，它有两个邻居：节点 2 和 4 。
节点 2 的值是 2，它有两个邻居：节点 1 和 3 。
节点 3 的值是 3，它有两个邻居：节点 2 和 4 。
节点 4 的值是 4，它有两个邻居：节点 1 和 3 。
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/02/01/graph.png)
<pre>
<strong>输入:</strong> adjList = [[]]
<strong>输出:</strong> [[]]
<strong>解释:</strong> 输入包含一个空列表。该图仅仅只有一个值为 1 的节点，它没有任何邻居。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> adjList = []
<strong>输出:</strong> []
<strong>解释:</strong> 这个图是空的，它不含任何节点。
</pre>

#### 示例 4:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/02/01/graph-1.png)
<pre>
<strong>输入:</strong> adjList = [[2],[1]]
<strong>输出:</strong> [[2],[1]]
</pre>

#### 提示:
1. 节点数不超过 100 。
2. 每个节点值 `Node.val` 都是唯一的，`1 <= Node.val <= 100`。
3. 无向图是一个[简单图](https://baike.baidu.com/item/%E7%AE%80%E5%8D%95%E5%9B%BE/1680528?fr=aladdin)，这意味着图中没有重复的边，也没有自环。
4. 由于图是无向的，如果节点 *p* 是节点 *q* 的邻居，那么节点 *q* 也必须是节点 *p* 的邻居。
5. 图是连通图，你可以从给定节点访问到所有节点。

## 题解 (Python)

### 1. 题解
```Python
"""
# Definition for a Node.
class Node:
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []
"""

class Solution:
    def cloneGraph(self, node: 'Node') -> 'Node':
        if not node:
            return None

        nodes = [node]
        visited = [False] * 101
        visited[node.val] = True
        i = 0

        while i < len(nodes):
            node = nodes[i]

            for neighbor in node.neighbors:
                if not visited[neighbor.val]:
                    visited[neighbor.val] = True
                    nodes.append(neighbor)
            node.neighbors.append(Node(node.val))

            i += 1

        for node in nodes:
            copy = node.neighbors[-1]
            for neighbor in node.neighbors[:-1]:
                copy.neighbors.append(neighbor.neighbors[-1])

        copy = nodes[0].neighbors[-1]

        for node in nodes:
            node.neighbors.pop()

        return copy
```
