# 968. Binary Tree Cameras
You are given the `root` of a binary tree. We install cameras on the tree nodes where each camera at a node can monitor its parent, itself, and its immediate children.

Return *the minimum number of cameras needed to monitor all nodes of the tree*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2018/12/29/bst_cameras_01.png)
<pre>
<strong>Input:</strong> root = [0,0,null,0,0]
<strong>Output:</strong> 1
<strong>Explanation:</strong> One camera is enough to monitor all nodes if placed as shown.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2018/12/29/bst_cameras_02.png)
<pre>
<strong>Input:</strong> root = [0,0,null,0,null,0,null,null,0]
<strong>Output:</strong> 2
<strong>Explanation:</strong> At least two cameras are needed to monitor all nodes of the tree. The above image shows one of the valid configurations of camera placement.
</pre>

#### Constraints:
* The number of nodes in the tree is in the range `[1, 1000]`.
* `Node.val == 0`

## Solutions (Python)

### 1. Solution
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

from functools import cache


class Solution:
    @cache
    def minCameraCover(self, root: Optional[TreeNode]) -> int:
        if root is None:
            return 0

        ret = self.coverBySelf(root)

        if root.left is not None:
            ret = min(ret, self.coverBySelf(root.left) +
                      self.minCameraCover(root.right))
        if root.right is not None:
            ret = min(ret, self.coverBySelf(root.right) +
                      self.minCameraCover(root.left))

        return ret

    @cache
    def coverBySelf(self, root: TreeNode) -> int:
        ret = 1
        if root.left is not None:
            ret += self.coverByParent(root.left)
        if root.right is not None:
            ret += self.coverByParent(root.right)

        return ret

    @cache
    def coverByParent(self, root: TreeNode) -> int:
        return min(self.coverBySelf(root), self.minCameraCover(root.left) + self.minCameraCover(root.right))
```
