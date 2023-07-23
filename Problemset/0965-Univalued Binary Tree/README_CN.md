# 965. 单值二叉树
如果二叉树每个节点都具有相同的值，那么该二叉树就是*单值*二叉树。

只有给定的树是单值二叉树时，才返回 ```true```；否则返回 ```false```。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/12/29/screen-shot-2018-12-25-at-50104-pm.png)
<pre>
<strong>输入:</strong> [1,1,1,1,1,null,1]
<strong>输出:</strong> true
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/12/29/screen-shot-2018-12-25-at-50050-pm.png)
<pre>
<strong>输入:</strong> [2,2,2,5,2]
<strong>输出:</strong> false
</pre>

#### 提示:
1. 给定树的节点数范围是 ```[1, 100]```。
2. 每个节点的值都是整数，范围为 ```[0, 99]``` 。

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
    def isUnivalTree(self, root: TreeNode) -> bool:
        left, right = False, False
        if not root.left:
            left = True
        elif root.val == root.left.val:
            left = self.isUnivalTree(root.left)
        if not root.right:
            right = True
        elif root.val == root.right.val:
            right = self.isUnivalTree(root.right)
        return left and right
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
    def isUnivalTree(self, root: TreeNode) -> bool:
        nodes = [root]
        i = 0
        while i < len(nodes):
            if nodes[i].val != root.val:
                return False
            if nodes[i].left:
                nodes.append(nodes[i].left)
            if nodes[i].right:
                nodes.append(nodes[i].right)
            i += 1
        return True
```
