# 1038. Binary Search Tree to Greater Sum Tree
Given the root of a binary **search** tree with distinct values, modify it so that every ```node``` has a new value equal to the sum of the values of the original tree that are greater than or equal to ```node.val```.

As a reminder, a *binary search tree* is a tree that satisfies these constraints:
* The left subtree of a node contains only nodes with keys **less than** the node's key.
* The right subtree of a node contains only nodes with keys **greater than** the node's key.
* Both the left and right subtrees must also be binary search trees.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/05/02/tree.png)
<pre>
<strong>Input:</strong> [4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]
<strong>Output:</strong> [30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]
</pre>

#### Note:
1. The number of nodes in the tree is between ```1``` and ```100```.
2. Each node will have value between ```0``` and ```100```.
3. The given tree is a binary search tree.

## Solutions (Python)

### 1. Inorder Traversal
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def bstToGst(self, root: TreeNode) -> TreeNode:
        stack = []
        curr = root
        sum = 0

        while stack or curr:
            while curr:
                stack.append(curr)
                curr = curr.right

            curr = stack.pop()
            sum += curr.val
            curr.val = sum

            curr = curr.left

        return root
```
