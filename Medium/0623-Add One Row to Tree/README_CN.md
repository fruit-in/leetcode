# 623. 在二叉树中增加一行
给定一个二叉树，根节点为第1层，深度为 1。在其第 ```d``` 层追加一行值为 ```v``` 的节点。

添加规则：给定一个深度值 ```d``` （正整数），针对深度为 ```d-1``` 层的每一**非空**节点 ```N```，为 ```N``` 创建两个值为 ```v``` 的左子树和右子树。

将 ```N``` 原先的左子树，连接为新节点 ```v``` 的左子树；将 ```N``` 原先的右子树，连接为新节点 ```v``` 的右子树。

如果 ```d``` 的值为 1，深度 d - 1 不存在，则创建一个新的根节点 ```v```，原先的整棵树将作为 ```v``` 的左子树。

#### 示例 1:
<pre>
<strong>输入:</strong>
二叉树如下所示:
       4
     /   \
    2     6
   / \   /
  3   1 5

<strong>v = 1</strong>

<strong>d = 2</strong>
<strong>输出:</strong>
       4
      / \
     1   1
    /     \
   2       6
  / \     /
 3   1   5
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong>
二叉树如下所示:
      4
     /
    2
   / \
  3   1

<strong>v = 1</strong>

<strong>d = 3</strong>
<strong>输出:</strong>
      4
     /
    2
   / \
  1   1
 /     \
3       1
</pre>

#### 注意:
1. 输入的深度值 d 的范围是：[1，二叉树最大深度 + 1]。
2. 输入的二叉树至少有一个节点。

## 题解 (Python)

### 1. 深度优先搜索
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def addOneRow(self, root: TreeNode, v: int, d: int) -> TreeNode:
        if d == 1:
            return TreeNode(v, root)
        elif d == 2:
            root.left = TreeNode(v, root.left)
            root.right = TreeNode(v, None, root.right)
        else:
            if root.left:
                self.addOneRow(root.left, v, d - 1)
            if root.right:
                self.addOneRow(root.right, v, d - 1)

        return root
```

### 2. 广度优先搜索
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def addOneRow(self, root: TreeNode, v: int, d: int) -> TreeNode:
        if d == 1:
            return TreeNode(v, root)

        curr_level = [root]
        for _ in range(d - 2):
            next_level = [node.left for node in curr_level if node.left]
            next_level.extend(node.right for node in curr_level if node.right)
            curr_level = next_level

        for node in curr_level:
            node.left = TreeNode(v, node.left)
            node.right = TreeNode(v, None, node.right)

        return root
```
