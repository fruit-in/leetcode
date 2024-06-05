# 1080. Insufficient Nodes in Root to Leaf Paths
Given the `root` of a binary tree and an integer `limit`, delete all **insufficient nodes** in the tree simultaneously, and return *the root of the resulting binary tree*.

A node is **insufficient** if every root to **leaf** path intersecting this node has a sum strictly less than `limit`.

A **leaf** is a node with no children.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/06/05/insufficient-11.png)
<pre>
<strong>Input:</strong> root = [1,2,3,4,-99,-99,7,8,9,-99,-99,12,13,-99,14], limit = 1
<strong>Output:</strong> [1,2,3,4,null,null,7,8,9,null,14]
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2019/06/05/insufficient-3.png)
<pre>
<strong>Input:</strong> root = [5,4,8,11,null,17,4,7,1,null,null,5,3], limit = 22
<strong>Output:</strong> [5,4,8,11,null,17,4,7,null,null,null,5]
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2019/06/11/screen-shot-2019-06-11-at-83301-pm.png)
<pre>
<strong>Input:</strong> root = [1,2,-3,-5,null,4,null], limit = -1
<strong>Output:</strong> [1,null,-3,4]
</pre>

#### Constraints:
* The number of nodes in the tree is in the range `[1, 5000]`.
* <code>-10<sup>5</sup> <= Node.val <= 10<sup>5</sup></code>
* <code>-10<sup>9</sup> <= limit <= 10<sup>9</sup></code>

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
    def sufficientSubset(self, root: Optional[TreeNode], limit: int) -> Optional[TreeNode]:
        def dfs(root: Optional[TreeNode]) -> None:
            if root is not None:
                if root.left is not None:
                    root.left.rootsum = root.rootsum + root.left.val
                if root.right is not None:
                    root.right.rootsum = root.rootsum + root.right.val

                dfs(root.left)
                dfs(root.right)

                if root.left is None and root.right is None:
                    root.leafsum = 0
                elif root.left is None:
                    root.leafsum = root.right.leafsum + root.right.val
                elif root.right is None:
                    root.leafsum = root.left.leafsum + root.left.val
                else:
                    root.leafsum = max(
                        root.left.leafsum + root.left.val, root.right.leafsum + root.right.val)

        root.rootsum = root.val
        dfs(root)

        if root.rootsum + root.leafsum < limit:
            return None

        nodes = [root]

        while nodes != []:
            node = nodes.pop()
            if node.left is not None:
                if node.left.rootsum + node.left.leafsum < limit:
                    node.left = None
                else:
                    nodes.append(node.left)
            if node.right is not None:
                if node.right.rootsum + node.right.leafsum < limit:
                    node.right = None
                else:
                    nodes.append(node.right)

        return root
```
