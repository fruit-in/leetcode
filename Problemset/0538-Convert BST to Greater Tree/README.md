# 538. Convert BST to Greater Tree
Given a Binary Search Tree (BST), convert it to a Greater Tree such that every key of the original BST is changed to the original key plus sum of all keys greater than the original key in BST.

#### Example:
<pre>
<strong>Input:</strong> The root of a Binary Search Tree like this:
              5
            /   \
           2     13
<strong>Output:</strong> The root of a Greater Tree like this:
             18
            /   \
          20     13
</pre>

## Solutions (Python)

### 1. Inorder Traversal
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def convertBST(self, root: TreeNode) -> TreeNode:
        nodes = []
        curr = root
        sum = 0

        while nodes or curr:
            while curr:
                nodes.append(curr)
                curr = curr.right

            curr = nodes.pop()

            sum += curr.val
            curr.val = sum

            curr = curr.left

        return root
```
