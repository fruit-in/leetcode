# 106. Construct Binary Tree from Inorder and Postorder Traversal
Given inorder and postorder traversal of a tree, construct the binary tree.

#### Note:
You may assume that duplicates do not exist in the tree.

For example, given
```
inorder = [9,3,15,20,7]
postorder = [9,15,7,20,3]
```

Return the following binary tree:
```
    3
   / \
  9  20
    /  \
   15   7
```

## Solutions (Python)

### 1. Recursion
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def buildTree(self, inorder: List[int], postorder: List[int]) -> TreeNode:
        if not postorder:
            return None

        i = inorder.index(postorder[-1])

        return TreeNode(
            postorder[-1],
            self.buildTree(inorder[:i], postorder[:i]),
            self.buildTree(inorder[i + 1:], postorder[i:-1])
        )
```
