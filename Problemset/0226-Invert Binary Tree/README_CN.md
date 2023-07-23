# 226. 翻转二叉树
翻转一棵二叉树。

#### 示例:
输入:
<pre>
     4
   /   \
  2     7
 / \   / \
1   3 6   9
</pre>
输出:
<pre>
     4
   /   \
  7     2
 / \   / \
9   6 3   1
</pre>

#### 备注:
这个问题是受到 [Max Howell](https://twitter.com/mxcl) 的 [原问题](https://twitter.com/mxcl/status/608682016205344768) 启发的 ：
> 谷歌：我们90％的工程师使用您编写的软件(Homebrew)，但是您却无法在面试时在白板上写出翻转二叉树这道题，这太糟糕了。

## 题解 (Python)

### 1. 深度优先搜索
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def invertTree(self, root: TreeNode) -> TreeNode:
        if root:
            root.left, root.right = self.invertTree(root.right), self.invertTree(root.left)
        return root
```

### 2. 广度优先搜索
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def invertTree(self, root: TreeNode) -> TreeNode:
        nodes = [root]
        i = 0
        while i < len(nodes):
            if nodes[i]:
                nodes.append(nodes[i].left)
                nodes.append(nodes[i].right)
                nodes[i].left = nodes[-1]
                nodes[i].right = nodes[-2]
            i += 1
        return root
```
