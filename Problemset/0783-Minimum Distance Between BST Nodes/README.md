# 783. Minimum Distance Between BST Nodes
Given a Binary Search Tree (BST) with the root node ```root```, return the minimum difference between the values of any two different nodes in the tree.

#### Example:
<pre>
<strong>Input:</strong> root = [4,2,6,1,3,null,null]
<strong>Output:</strong> 1
<strong>Explanation:</strong>
Note that root is a TreeNode object, not an array.

The given tree [4,2,6,1,3,null,null] is represented by the following diagram:

          4
        /   \
      2      6
     / \
    1   3

while the minimum difference in this tree is 1, it occurs between node 1 and node 2, also between node 3 and node 2.
</pre>

#### Note:
1. The size of the BST will be between 2 and ```100```.
2. The BST is always valid, each node's value is an integer, and each node's value is different.

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
    def minDiffInBST(self, root: TreeNode) -> int:
        nodes = []
        curr = root
        prev = float("-inf")
        min_diff = float("+inf")

        while nodes or curr:
            while curr:
                nodes.append(curr)
                curr = curr.left

            curr = nodes.pop()

            min_diff = min(min_diff, curr.val - prev)
            prev = curr.val

            curr = curr.right

        return min_diff
```
