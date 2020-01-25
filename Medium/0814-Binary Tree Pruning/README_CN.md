# 814. 二叉树剪枝
给定二叉树根结点 ```root``` ，此外树的每个结点的值要么是 0，要么是 1。

返回移除了所有不包含 1 的子树的原二叉树。

( 节点 X 的子树为 X 本身，以及所有 X 的后代。)

<pre>
<strong>示例1:</strong>
<strong>输入:</strong> [1,null,0,0,1]
<strong>输出:</strong> [1,null,0,null,1]
<strong>解释:</strong>
只有红色节点满足条件“所有不包含 1 的子树”。
右图为返回的答案。
<img src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/04/06/1028_2.png">
</pre>

<pre>
<strong>示例2:</strong>
<strong>输入:</strong> [1,0,1,0,0,0,1]
<strong>输出:</strong> [1,null,1,null,1]
<img src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/04/06/1028_1.png">
</pre>

<pre>
<strong>示例3:</strong>
<strong>输入:</strong> [1,1,0,1,1,0,1,0]
<strong>输出:</strong> [1,1,0,1,1,null,1]
<img src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/04/05/1028.png">
</pre>

#### 说明:
* 给定的二叉树最多有 ```100``` 个节点。
* 每个节点的值只会为 ```0``` 或 ```1``` 。

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
    def pruneTree(self, root: TreeNode) -> TreeNode:
        if not root:
            return None

        root.left = self.pruneTree(root.left)
        root.right = self.pruneTree(root.right)

        if root.val == 1 or root.left or root.right:
            return root
        else:
            return None
```
