# 129. Sum Root to Leaf Numbers
Given a binary tree containing digits from ```0-9``` only, each root-to-leaf path could represent a number.

An example is the root-to-leaf path ```1->2->3``` which represents the number ```123```.

Find the total sum of all root-to-leaf numbers.

**Note:** A leaf is a node with no children.

#### Example 1:
<pre>
<strong>Input:</strong> [1,2,3]
    1
   / \
  2   3
<strong>Output:</strong> 25
<strong>Explanation:</strong>
The root-to-leaf path 1->2 represents the number 12.
The root-to-leaf path 1->3 represents the number 13.
Therefore, sum = 12 + 13 = 25.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [4,9,0,5,1]
    4
   / \
  9   0
 / \
5   1
<strong>Output:</strong> 1026
<strong>Explanation:</strong>
The root-to-leaf path 4->9->5 represents the number 495.
The root-to-leaf path 4->9->1 represents the number 491.
The root-to-leaf path 4->0 represents the number 40.
Therefore, sum = 495 + 491 + 40 = 1026.
</pre>

## Solutions (Python)

### 1. DFS
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def sumNumbers(self, root: TreeNode) -> int:
        def helper(root: TreeNode, n: int) -> int:
            n = 10 * n + root.val

            if root.left and root.right:
                return helper(root.left, n) + helper(root.right, n)
            elif root.left:
                return helper(root.left, n)
            elif root.right:
                return helper(root.right, n)
            else:
                return n

        return helper(root, 0) if root else 0
```
