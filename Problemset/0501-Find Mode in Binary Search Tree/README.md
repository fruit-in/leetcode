# 501. Find Mode in Binary Search Tree
Given a binary search tree (BST) with duplicates, find all the [mode(s)](https://en.wikipedia.org/wiki/Mode_(statistics)) (the most frequently occurred element) in the given BST.

Assume a BST is defined as follows:
* The left subtree of a node contains only nodes with keys **less than or equal to** the node's key.
* The right subtree of a node contains only nodes with keys **greater than or equal to** the node's key.
* Both the left and right subtrees must also be binary search trees.

For example:<br>
Given BST ```[1,null,2,2]```,
```
   1
    \
     2
    /
   2
```
return ```[2]```.

**Note:** If a tree has more than one mode, you can return them in any order.

**Follow up:** Could you do that without using any extra space? (Assume that the implicit stack space incurred due to recursion does not count).

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
    def findMode(self, root: TreeNode) -> List[int]:
        nodes = []
        curr = root
        prev = 0
        cnt = 0
        max_cnt = 1
        modes = []

        while nodes or curr:
            while curr:
                nodes.append(curr)
                curr = curr.left

            curr = nodes.pop()

            if curr.val == prev:
                cnt += 1
            else:
                if cnt == max_cnt:
                    modes.append(prev)
                elif cnt > max_cnt:
                    modes = [prev]
                    max_cnt = cnt

                prev = curr.val
                cnt = 1

            curr = curr.right

        if cnt == max_cnt:
            modes.append(prev)
        elif cnt > max_cnt:
            modes = [prev]

        return modes
```
