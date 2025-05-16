# 2458. Height of Binary Tree After Subtree Removal Queries
You are given the `root` of a **binary tree** with `n` nodes. Each node is assigned a unique value from `1` to `n`. You are also given an array `queries` of size `m`.

You have to perform `m` **independent** queries on the tree where in the ith query you do the following:
* **Remove** the subtree rooted at the node with the value `queries[i]` from the tree. It is **guaranteed** that `queries[i]` will **not** be equal to the value of the root.

Return *an array* `answer` *of size* `m` *where* `answer[i]` *is the height of the tree after performing the* <code>i<sup>th</sup></code> *query*.

**Note**:
* The queries are independent, so the tree returns to its **initial** state after each query.
* The height of a tree is the **number of edges in the longest simple path** from the root to some node in the tree.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/09/07/binaryytreeedrawio-1.png)
<pre>
<strong>Input:</strong> root = [1,3,4,2,null,6,5,null,null,null,null,null,7], queries = [4]
<strong>Output:</strong> [2]
<strong>Explanation:</strong> The diagram above shows the tree after removing the subtree rooted at node with value 4.
The height of the tree is 2 (The path 1 -> 3 -> 2).
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/09/07/binaryytreeedrawio-2.png)
<pre>
<strong>Input:</strong> root = [5,8,9,2,1,3,7,4,6], queries = [3,2,4,8]
<strong>Output:</strong> [3,2,3,2]
<strong>Explanation:</strong> We have the following queries:
- Removing the subtree rooted at node with value 3. The height of the tree becomes 3 (The path 5 -> 8 -> 2 -> 4).
- Removing the subtree rooted at node with value 2. The height of the tree becomes 2 (The path 5 -> 8 -> 1).
- Removing the subtree rooted at node with value 4. The height of the tree becomes 3 (The path 5 -> 8 -> 2 -> 6).
- Removing the subtree rooted at node with value 8. The height of the tree becomes 2 (The path 5 -> 9 -> 3).
</pre>

#### Constraints:
* The number of nodes in the tree is `n`.
* <code>2 <= n <= 10<sup>5</sup></code>
* `1 <= Node.val <= n`
* All the values in the tree are **unique**.
* `m == queries.length`
* <code>1 <= m <= min(n, 10<sup>4</sup>)</code>
* `1 <= queries[i] <= n`
* `queries[i] != root.val`

## Solutions (Python)

### 1. Solution
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
