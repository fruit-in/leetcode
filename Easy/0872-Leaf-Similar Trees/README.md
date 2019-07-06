# 872. Leaf-Similar Trees
Consider all the leaves of a binary tree.  From left to right order, the values of those leaves form a *leaf value sequence*.

![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/16/tree.png)

For example, in the given tree above, the leaf value sequence is <code>(6, 7, 4, 9, 8)</code>.

Two binary trees are considered *leaf-similar* if their leaf value sequence is the same.

Return <code>true</code> if and only if the two given trees with head nodes <code>root1</code> and <code>root2</code> are leaf-similar.

#### Note:
* Both of the given trees will have between <code>1</code> and <code>100</code> nodes.

## Solutions

### 1. DFS (Python3)
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def leafSimilar(self, root1: TreeNode, root2: TreeNode) -> bool:
        def getLeaves(root: TreeNode) -> List[int]:
            if not root.left and not root.right:
                return [root.val]
            leaves = []
            if root.left:
                leaves += getLeaves(root.left)
            if root.right:
                leaves += getLeaves(root.right)
            return leaves

        return getLeaves(root1) == getLeaves(root2)
```
