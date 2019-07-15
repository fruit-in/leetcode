# 993. Cousins in Binary Tree
In a binary tree, the root node is at depth <code>0</code>, and children of each depth <code>k</code> node are at depth <code>k+1</code>.

Two nodes of a binary tree are *cousins* if they have the same depth, but have **different parents**.

We are given the <code>root</code> of a binary tree with unique values, and the values <code>x</code> and <code>y</code> of two different nodes in the tree.

Return <code>true</code> if and only if the nodes corresponding to the values <code>x</code> and <code>y</code> are cousins.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/02/12/q1248-01.png)
<pre>
<strong>Input:</strong> root = [1,2,3,4], x = 4, y = 3
<strong>Output:</strong> false
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2019/02/12/q1248-02.png)
<pre>
<strong>Input:</strong> root = [1,2,3,null,4,null,5], x = 5, y = 4
<strong>Output:</strong> true
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2019/02/13/q1248-03.png)
<pre>
<strong>Input:</strong> root = [1,2,3,null,4], x = 2, y = 3
<strong>Output:</strong> false
</pre>

#### Note:
1. The number of nodes in the tree will be between <code>2</code> and <code>100</code>.
2. Each node has a unique integer value from <code>1</code> to <code>100</code>.

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
    def isCousins(self, root: TreeNode, x: int, y: int) -> bool:
        parent, depth = {root.val: None}, {}
        def dfs(root: TreeNode, dep: int):
            depth[root.val] = dep
            if root.left:
                parent[root.left.val] = root
                dfs(root.left, dep + 1)
            if root.right:
                parent[root.right.val] = root
                dfs(root.right, dep + 1)

        dfs(root, 0)
        return depth[x] == depth[y] and parent[x] != parent[y]
```

### 2. BFS (Python3)
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def isCousins(self, root: TreeNode, x: int, y: int) -> bool:
        nodes = [root]
        while nodes:
            nodes = [node.left for node in nodes if node] \
                + [node.right for node in nodes if node]
            vals = [node.val if node else 0 for node in nodes]
            if x in vals and y in vals:
                return abs(vals.index(x) - vals.index(y)) != len(vals) / 2
            elif x in vals or y in vals:
                return False
```
