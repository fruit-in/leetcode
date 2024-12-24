# 968. 监控二叉树
给定一个二叉树，我们在树的节点上安装摄像头。

节点上的每个摄影头都可以监视**其父对象、自身及其直接子对象**。

计算监控树的所有节点所需的最小摄像头数量。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2018/12/29/bst_cameras_01.png)
<pre>
<strong>输入:</strong> root = [0,0,null,0,0]
<strong>输出:</strong> 1
<strong>解释:</strong> 如图所示，一台摄像头足以监控所有节点。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2018/12/29/bst_cameras_02.png)
<pre>
<strong>输入:</strong> root = [0,0,null,0,null,0,null,null,0]
<strong>输出:</strong> 2
<strong>解释:</strong> 需要至少两个摄像头来监视树的所有节点。 上图显示了摄像头放置的有效位置之一。
</pre>

#### 提示:
1. 给定树的节点数的范围是 `[1, 1000]`。
2. 每个节点的值都是 0。

## 题解 (Python)

### 1. 题解
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
