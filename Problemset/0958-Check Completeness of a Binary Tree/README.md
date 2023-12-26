# 958. Check Completeness of a Binary Tree
Given the `root` of a binary tree, determine if it is a *complete binary tree*.

In a [**complete binary**](http://en.wikipedia.org/wiki/Binary_tree#Types_of_binary_trees) tree, every level, except possibly the last, is completely filled, and all nodes in the last level are as far left as possible. It can have between `1` and <code>2<sup>h</sup></code> nodes inclusive at the last level `h`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2018/12/15/complete-binary-tree-1.png)
<pre>
<strong>Input:</strong> root = [1,2,3,4,5,6]
<strong>Output:</strong> true
<strong>Explanation:</strong> Every level before the last is full (ie. levels with node-values {1} and {2, 3}), and all nodes in the last level ({4, 5, 6}) are as far left as possible.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2018/12/15/complete-binary-tree-2.png)
<pre>
<strong>Input:</strong> root = [1,2,3,4,5,null,7]
<strong>Output:</strong> false
<strong>Explanation:</strong> The node with value 7 isn't as far left as possible.
</pre>

#### Constraints:
* The number of nodes in the tree is in the range `[1, 100]`.
* `1 <= Node.val <= 1000`

## Solutions (Python)

### 1. Solution
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def isCompleteTree(self, root: Optional[TreeNode]) -> bool:
        nodes = [root]
        depth = 0

        while True:
            children = []

            if nodes[0] == None:
                return True

            if len(nodes) != 2 ** depth:
                return False

            for node in nodes:
                if node is not None:
                    children.append(node.left)
                    children.append(node.right)

            for i in range(1, len(children)):
                if children[i - 1] is None and children[i] is not None:
                    return False

            nodes = children
            depth += 1
```
