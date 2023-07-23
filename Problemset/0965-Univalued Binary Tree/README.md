# 965. Univalued Binary Tree
A binary tree is *univalued* if every node in the tree has the same value.

Return <code>true</code> if and only if the given tree is univalued.

#### Example 1:
![](https://assets.leetcode.com/uploads/2018/12/28/unival_bst_1.png)
<pre>
<strong>Input:</strong> [1,1,1,1,1,null,1]
<strong>Output:</strong> true
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2018/12/28/unival_bst_2.png)
<pre>
<strong>Input:</strong> [2,2,2,5,2]
<strong>Output:</strong> false
</pre>

#### Note:
1. The number of nodes in the given tree will be in the range <code>[1, 100]</code>.
2. Each node's value will be an integer in the range <code>[0, 99]</code>.

## Solutions (Python)

### 1. DFS
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

### 2. BFS
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
