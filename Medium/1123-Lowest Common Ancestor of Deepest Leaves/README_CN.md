# 1123. 最深叶节点的最近公共祖先
给你一个有根节点的二叉树，找到它最深的叶节点的最近公共祖先。

回想一下：
* **叶节点** 是二叉树中没有子节点的节点
* 树的根节点的 **深度** 为 ```0```，如果某一节点的深度为 ```d```，那它的子节点的深度就是 ```d+1```
* 如果我们假定 ```A``` 是一组节点 ```S``` 的 **最近公共祖先**，```S``` 中的每个节点都在以 ```A``` 为根节点的子树中，且 ```A``` 的深度达到此条件下可能的最大值。

#### 示例 1:
<pre>
<strong>输入:</strong> root = [1,2,3]
<strong>输出:</strong> [1,2,3]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> root = [1,2,3,4]
<strong>输出:</strong> [4]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> root = [1,2,3,4,5]
<strong>输出:</strong> [2,4,5]
</pre>

#### 提示:
* 给你的树中将有 1 到 1000 个节点。
* 树中每个节点的值都在 1 到 1000 之间。

## 题解 (Python)

### 1. 递归
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def lcaDeepestLeaves(self, root: TreeNode) -> TreeNode:
        def helper(root: TreeNode, depth: int) -> (TreeNode, int):
            if not root:
                return (None, depth - 1)

            l_node, l_depth = helper(root.left, depth + 1)
            r_node, r_depth = helper(root.right, depth + 1)

            if l_depth > r_depth:
                return (l_node, l_depth)
            elif l_depth < r_depth:
                return (r_node, r_depth)
            else:
                return (root, l_depth)

        return helper(root, 0)[0]
```
