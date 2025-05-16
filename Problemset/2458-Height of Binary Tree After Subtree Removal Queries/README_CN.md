# 2458. 移除子树后的二叉树高度
给你一棵 **二叉树** 的根节点 `root` ，树中有 `n` 个节点。每个节点都可以被分配一个从 `1` 到 `n` 且互不相同的值。另给你一个长度为 `m` 的数组 `queries` 。

你必须在树上执行 `m` 个 **独立** 的查询，其中第 `i` 个查询你需要执行以下操作：
* 从树中 **移除** 以 `queries[i]` 的值作为根节点的子树。题目所用测试用例保证 `queries[i]` **不** 等于根节点的值。

返回一个长度为 `m` 的数组 `answer` ，其中 `answer[i]` 是执行第 `i` 个查询后树的高度。

**注意：**
* 查询之间是独立的，所以在每个查询执行后，树会回到其 **初始** 状态。
* 树的高度是从根到树中某个节点的 **最长简单路径中的边数** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/09/07/binaryytreeedrawio-1.png)
<pre>
<strong>输入:</strong> root = [1,3,4,2,null,6,5,null,null,null,null,null,7], queries = [4]
<strong>输出:</strong> [2]
<strong>解释:</strong> 上图展示了从树中移除以 4 为根节点的子树。
树的高度是 2（路径为 1 -> 3 -> 2）。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/09/07/binaryytreeedrawio-2.png)
<pre>
<strong>输入:</strong> root = [5,8,9,2,1,3,7,4,6], queries = [3,2,4,8]
<strong>输出:</strong> [3,2,3,2]
<strong>解释:</strong> 执行下述查询：
- 移除以 3 为根节点的子树。树的高度变为 3（路径为 5 -> 8 -> 2 -> 4）。
- 移除以 2 为根节点的子树。树的高度变为 2（路径为 5 -> 8 -> 1）。
- 移除以 4 为根节点的子树。树的高度变为 3（路径为 5 -> 8 -> 2 -> 6）。
- 移除以 8 为根节点的子树。树的高度变为 2（路径为 5 -> 9 -> 3）。
</pre>

#### 提示:
* 树中节点的数目是 `n`
* <code>2 <= n <= 10<sup>5</sup></code>
* `1 <= Node.val <= n`
* 树中的所有值 **互不相同**
* `m == queries.length`
* <code>1 <= m <= min(n, 10<sup>4</sup>)</code>
* `1 <= queries[i] <= n`
* `queries[i] != root.val`

## 题解 (Python)

### 1. 题解
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def treeQueries(self, root: Optional[TreeNode], queries: List[int]) -> List[int]:
        def dfs(root: TreeNode) -> int:
            nodes[root.val] = root
            root.height = 0

            if root.level >= len(leveltop2):
                leveltop2.append([])

            if root.left:
                root.left.level = root.level + 1
                root.height = dfs(root.left) + 1
            if root.right:
                root.right.level = root.level + 1
                root.height = max(root.height, dfs(root.right) + 1)

            leveltop2[root.level].append(root.height)
            leveltop2[root.level] = sorted(leveltop2[root.level])[-2:]

            return root.height

        nodes = {}
        leveltop2 = []
        root.level = 0
        answer = [0] * len(queries)

        dfs(root)

        for i in range(len(queries)):
            node = nodes[queries[i]]
            if len(leveltop2[node.level]) < 2:
                answer[i] = node.level - 1
            elif leveltop2[node.level][1] != node.height:
                answer[i] = leveltop2[node.level][1] + node.level
            else:
                answer[i] = leveltop2[node.level][0] + node.level

        return answer
```
