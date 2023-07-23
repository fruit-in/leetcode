# 897. Increasing Order Search Tree
Given a binary search tree, rearrange the tree in **in-order** so that the leftmost node in the tree is now the root of the tree, and every node has no left child and only 1 right child.

<pre>
<strong>Example 1:</strong>
<strong>Input:</strong> [5,3,6,2,4,null,8,1,null,null,null,7,9]

       5
      / \
    3    6
   / \    \
  2   4    8
 /        / \
1        7   9

<strong>Output:</strong> [1,null,2,null,3,null,4,null,5,null,6,null,7,null,8,null,9]

 1
  \
   2
    \
     3
      \
       4
        \
         5
          \
           6
            \
             7
              \
               8
                \
                 9
</pre>

#### Note:
1. The number of nodes in the given tree will be between 1 and 100.
2. Each node will have a unique integer value from 0 to 1000.

## Solutions (Python)

### 1. Inorder Traversal(Build A New Tree)
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def increasingBST(self, root: TreeNode) -> TreeNode:
        root_parent = TreeNode(0)
        prev = root_parent
        nodes = []
        curr = root

        while nodes or curr:
            while curr:
                nodes.append(curr)
                curr = curr.left

            curr = nodes.pop()

            prev.right = curr
            curr.left = None
            prev = curr

            curr = curr.right

        return root_parent.right
```

### 2. Inorder Traversal(Relinking)
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def increasingBST(self, root: TreeNode) -> TreeNode:
        prev = None
        nodes = []
        curr = root

        while nodes or curr:
            while curr:
                nodes.append(curr)
                curr = curr.right

            curr = nodes.pop()

            new_node = TreeNode(curr.val)
            new_node.right = prev
            prev = new_node

            curr = curr.left

        return prev
```
