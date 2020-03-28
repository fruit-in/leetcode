# 865. Smallest Subtree with all the Deepest Nodes
Given a binary tree rooted at ```root```, the *depth* of each node is the shortest distance to the root.

A node is *deepest* if it has the largest depth possible among any node in the <u>entire tree</u>.

The subtree of a node is that node, plus the set of all descendants of that node.

Return the node with the largest depth such that it contains all the deepest nodes in its subtree.

#### Example 1:
<pre>
<strong>Input:</strong> [3,5,1,6,2,0,8,null,null,7,4]
<strong>Output:</strong> [2,7,4]
<strong>Explanation:</strong>
<img src='https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/01/sketch1.png'>
We return the node with value 2, colored in yellow in the diagram.
The nodes colored in blue are the deepest nodes of the tree.
The input "[3, 5, 1, 6, 2, 0, 8, null, null, 7, 4]" is a serialization of the given tree.
The output "[2, 7, 4]" is a serialization of the subtree rooted at the node with value 2.
Both the input and output have TreeNode type.
</pre>

#### Note:
* The number of nodes in the tree will be between 1 and 500.
* The values of each node are unique.

## Solutions (Python)

### 1. Recursion
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def subtreeWithAllDeepest(self, root: TreeNode) -> TreeNode:
        def helper(root: TreeNode) -> (TreeNode, int):
            if not root:
                return None, -1

            l_node, l_h = helper(root.left)
            r_node, r_h = helper(root.right)

            if l_h > r_h:
                return l_node, l_h + 1
            elif l_h < r_h:
                return r_node, r_h + 1
            else:
                return root, l_h + 1

        return helper(root)[0]
```
