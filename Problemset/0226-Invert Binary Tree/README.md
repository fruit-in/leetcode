# 226. Invert Binary Tree
Invert a binary tree.

#### Example:
Input:
<pre>
     4
   /   \
  2     7
 / \   / \
1   3 6   9
</pre>
Output:
<pre>
     4
   /   \
  7     2
 / \   / \
9   6 3   1
</pre>

#### Trivia:
This problem was inspired by <code>this original tweet</code> by <code>Max Howell</code>:
<pre>
Google: 90% of our engineers use the software you wrote (Homebrew),
but you can’t invert a binary tree on a whiteboard so f*** off.
</pre>

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
    def invertTree(self, root: TreeNode) -> TreeNode:
        if root:
            root.left, root.right = self.invertTree(root.right), self.invertTree(root.left)
        return root
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
