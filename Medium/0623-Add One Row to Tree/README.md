# 623. Add One Row to Tree
Given the root of a binary tree, then value ```v``` and depth ```d```, you need to add a row of nodes with value ```v``` at the given depth ```d```. The root node is at depth 1.

The adding rule is: given a positive integer depth ```d```, for each NOT null tree nodes ```N``` in depth ```d-1```, create two tree nodes with value ```v``` as ```N's``` left subtree root and right subtree root. And ```N's``` **original left subtree** should be the left subtree of the new left subtree root, its **original right subtree** should be the right subtree of the new right subtree root. If depth ```d``` is 1 that means there is no depth d-1 at all, then create a tree node with value **v** as the new root of the whole original tree, and the original tree is the new root's left subtree.

#### Example 1:
<pre>
<strong>Input:</strong>
A binary tree as following:
       4
     /   \
    2     6
   / \   /
  3   1 5

<strong>v = 1</strong>

<strong>d = 2</strong>
<strong>Output:</strong>
       4
      / \
     1   1
    /     \
   2       6
  / \     /
 3   1   5
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
A binary tree as following:
      4
     /
    2
   / \
  3   1

<strong>v = 1</strong>

<strong>d = 3</strong>
<strong>Output:</strong>
      4
     /
    2
   / \
  1   1
 /     \
3       1
</pre>

#### Note:
1. The given d is in range [1, maximum depth of the given tree + 1].
2. The given binary tree has at least one tree node.

## Solutions (Python)

### 1. DFS
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

### 2. BFS
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
