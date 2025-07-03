# 1483. 树节点的第 K 个祖先
给你一棵树，树上有 `n` 个节点，按从 `0` 到 `n-1` 编号。树以父节点数组的形式给出，其中 `parent[i]` 是节点 `i` 的父节点。树的根节点是编号为 `0` 的节点。

树节点的第 `k` 个祖先节点是从该节点到根节点路径上的第 `k` 个节点。

实现 `TreeAncestor` 类：
* `TreeAncestor（int n， int[] parent）` 对树和父数组中的节点数初始化对象。
* `getKthAncestor(int node, int k)` 返回节点 `node` 的第 `k` 个祖先节点。如果不存在这样的祖先节点，返回 `-1` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2019/08/28/1528_ex1.png)
<pre>
<strong>输入:</strong>
["TreeAncestor", "getKthAncestor", "getKthAncestor", "getKthAncestor"]
[[7, [-1, 0, 0, 1, 1, 2, 2]], [3, 1], [5, 2], [6, 3]]
<strong>输出:</strong>
[null, 1, 0, -1]
<strong>解释:</strong>
TreeAncestor treeAncestor = new TreeAncestor(7, [-1, 0, 0, 1, 1, 2, 2]);
treeAncestor.getKthAncestor(3, 1);  // 返回 1 ，它是 3 的父节点
treeAncestor.getKthAncestor(5, 2);  // 返回 0 ，它是 5 的祖父节点
treeAncestor.getKthAncestor(6, 3);  // 返回 -1 因为不存在满足要求的祖先节点
</pre>

#### 提示:
* <code>1 <= k <= n <= 5 * 10<sup>4</sup></code>
* `parent[0] == -1` 表示编号为 `0` 的节点是根节点。
* 对于所有的 `0 < i < n` ，`0 <= parent[i] < n` 总成立
* `0 <= node < n`
* 至多查询 <code>5 * 10<sup>4</sup></code> 次

## 题解 (Python)

### 1. 题解
```Python
class TreeAncestor:

    def __init__(self, n: int, parent: List[int]):
        def dfs(i: int):
            while 1 << len(self.ancestors[i]) <= self.depth[i]:
                self.ancestors[i].append(self.getKthAncestor(
                    parent[i], ((1 << len(self.ancestors[i])) - 1)))
            for child in children[i]:
                self.depth[child] = self.depth[i] + 1
                dfs(child)

        children = [[] for _ in range(n)]
        self.depth = [0] * n
        self.ancestors = [[] for _ in range(n)]
        for i in range(1, len(parent)):
            children[parent[i]].append(i)
        dfs(0)

    def getKthAncestor(self, node: int, k: int) -> int:
        if k == 0:
            return node
        if k > self.depth[node]:
            return -1

        return self.getKthAncestor(self.ancestors[node][k.bit_length() - 1], k ^ (1 << (k.bit_length() - 1)))


# Your TreeAncestor object will be instantiated and called as such:
# obj = TreeAncestor(n, parent)
# param_1 = obj.getKthAncestor(node,k)
```
