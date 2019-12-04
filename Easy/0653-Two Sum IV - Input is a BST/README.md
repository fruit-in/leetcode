# 653. Two Sum IV - Input is a BST
Given a Binary Search Tree and a target number, return true if there exist two elements in the BST such that their sum is equal to the given target.

#### Example 1:
<pre>
<strong>Input:</strong>
    5
   / \
  3   6
 / \   \
2   4   7

Target = 9
<strong>Output:</strong> True
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
    5
   / \
  3   6
 / \   \
2   4   7

Target = 28
<strong>Output:</strong> False
</pre>

## Solutions (Python)

### 1. Set
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def findTarget(self, root: TreeNode, k: int) -> bool:
        def helper(root: TreeNode, k: int, vals: set) -> bool:
            if not root:
                return False

            if k - root.val in vals:
                return True

            vals.add(root.val)

            return helper(root.left, k, vals) or helper(root.right, k, vals)

        return helper(root, k, set())
```

### 2. Inorder Traversal
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def findTarget(self, root: TreeNode, k: int) -> bool:
        nodes = []
        curr = root
        vals = []

        while nodes or curr:
            while curr:
                nodes.append(curr)
                curr = curr.left

            curr = nodes.pop()
            vals.append(curr.val)

            curr = curr.right

        l, r = 0, len(vals) - 1

        while l < r:
            if vals[l] + vals[r] < k:
                l += 1
            elif vals[l] + vals[r] > k:
                r -= 1
            else:
                return True

        return False
```
