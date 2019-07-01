# 700. Search in a Binary Search Tree
Given the root node of a binary search tree (BST) and a value. You need to find the node in the BST that the node's value equals the given value. Return the subtree rooted with that node. If such node doesn't exist, you should return NULL.

For example,
<pre>
Given the tree:
        4
       / \
      2   7
     / \
    1   3

And the value to search: 2
</pre>
You should return this subtree:
<pre>
      2
     / \
    1   3
</pre>

In the example above, if we want to search the value <code>5</code>, since there is no node with value <code>5</code>, we should return <code>NULL</code>.

Note that an empty tree is represented by <code>NULL</code>, therefore you would see the expected output (serialized tree format) as <code>[]</code>, not <code>null</code>.

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
    def searchBST(self, root: TreeNode, val: int) -> TreeNode:
        if not root:
            return None
        if root.val == val:
            return root
        elif root.val > val:
            return self.searchBST(root.left, val)
        elif root.val < val:
            return self.searchBST(root.right, val)
```
