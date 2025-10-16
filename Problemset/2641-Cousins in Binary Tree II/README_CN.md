# 2641. 二叉树的堂兄弟节点 II
给你一棵二叉树的根 `root` ，请你将每个节点的值替换成该节点的所有 **堂兄弟节点值的和** 。

如果两个节点在树中有相同的深度且它们的父节点不同，那么它们互为 **堂兄弟** 。

请你返回修改值之后，树的根 `root` 。

**注意**，一个节点的深度指的是从树根节点到这个节点经过的边数。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2023/01/11/example11.png)
<pre>
<strong>输入:</strong> root = [5,4,9,1,10,null,7]
<strong>输出:</strong> [0,0,0,7,7,null,11]
<strong>解释:</strong> 上图展示了初始的二叉树和修改每个节点的值之后的二叉树。
- 值为 5 的节点没有堂兄弟，所以值修改为 0 。
- 值为 4 的节点没有堂兄弟，所以值修改为 0 。
- 值为 9 的节点没有堂兄弟，所以值修改为 0 。
- 值为 1 的节点有一个堂兄弟，值为 7 ，所以值修改为 7 。
- 值为 10 的节点有一个堂兄弟，值为 7 ，所以值修改为 7 。
- 值为 7 的节点有两个堂兄弟，值分别为 1 和 10 ，所以值修改为 11 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2023/01/11/diagram33.png)
<pre>
<strong>输入:</strong> root = [3,1,2]
<strong>输出:</strong> [0,0,0]
<strong>解释:</strong> 上图展示了初始的二叉树和修改每个节点的值之后的二叉树。
- 值为 3 的节点没有堂兄弟，所以值修改为 0 。
- 值为 1 的节点没有堂兄弟，所以值修改为 0 。
- 值为 2 的节点没有堂兄弟，所以值修改为 0 。
</pre>

#### 提示:
* 树中节点数目的范围是 <code>[1, 10<sup>5</sup>]</code> 。
* <code>1 <= Node.val <= 10<sup>4</sup></code>

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
    def replaceValueInTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        def dfsLevelSums(root: Optional[TreeNode], depth: int) -> None:
            if not root:
                return

            while depth >= len(levelsums):
                levelsums.append(0)
            levelsums[depth] += root.val

            dfsLevelSums(root.left, depth + 1)
            dfsLevelSums(root.right, depth + 1)

        def dfsCousins(root: Optional[TreeNode], depth: int) -> None:
            if not root:
                return

            leftval = root.left.val if root.left else 0
            rightval = root.right.val if root.right else 0

            if root.left:
                root.left.val = levelsums[depth + 1] - leftval - rightval
                dfsCousins(root.left, depth + 1)
            if root.right:
                root.right.val = levelsums[depth + 1] - leftval - rightval
                dfsCousins(root.right, depth + 1)

        levelsums = []
        dfsLevelSums(root, 0)
        dfsCousins(root, 0)
        root.val = 0

        return root
```
