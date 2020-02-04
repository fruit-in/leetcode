# 1123. Lowest Common Ancestor of Deepest Leaves
Given a rooted binary tree, return the lowest common ancestor of its deepest leaves.

Recall that:
* The node of a binary tree is a *leaf* if and only if it has no children
* The *depth* of the root of the tree is 0, and if the depth of a node is ```d```, the depth of each of its children is ```d+1```.
* The *lowest common ancestor* of a set ```S``` of nodes is the node ```A``` with the largest depth such that every node in S is in the subtree with root ```A```.

#### Example 1:
<pre>
<strong>Input:</strong> root = [1,2,3]
<strong>Output:</strong> [1,2,3]
<strong>Explanation:</strong>
The deepest leaves are the nodes with values 2 and 3.
The lowest common ancestor of these leaves is the node with value 1.
The answer returned is a TreeNode object (not an array) with serialization "[1,2,3]".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> root = [1,2,3,4]
<strong>Output:</strong> [4]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> root = [1,2,3,4,5]
<strong>Output:</strong> [2,4,5]
</pre>

#### Constraints:
* The given tree will have between 1 and 1000 nodes.
* Each node of the tree will have a distinct value between 1 and 1000.

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
    def lcaDeepestLeaves(self, root: TreeNode) -> TreeNode:
        def helper(root: TreeNode, depth: int) -> (TreeNode, int):
            if not root:
                return (None, depth - 1)

            l_node, l_depth = helper(root.left, depth + 1)
            r_node, r_depth = helper(root.right, depth + 1)

            if l_depth > r_depth:
                return (l_node, l_depth)
            elif l_depth < r_depth:
                return (r_node, r_depth)
            else:
                return (root, l_depth)

        return helper(root, 0)[0]
```
