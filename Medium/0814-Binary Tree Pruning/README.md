# 814. Binary Tree Pruning
We are given the head node ```root``` of a binary tree, where additionally every node's value is either a 0 or a 1.

Return the same tree where every subtree (of the given tree) not containing a 1 has been removed.

(Recall that the subtree of a node X is X, plus every node that is a descendant of X.)

<pre>
<strong>Example 1:</strong>
<strong>Input:</strong> [1,null,0,0,1]
<strong>Output:</strong> [1,null,0,null,1]
<strong>Explanation:</strong>
Only the red nodes satisfy the property "every subtree not containing a 1".
The diagram on the right represents the answer.
<img src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/04/06/1028_2.png">
</pre>

<pre>
<strong>Example 2:</strong>
<strong>Input:</strong> [1,0,1,0,0,0,1]
<strong>Output:</strong> [1,null,1,null,1]
<img src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/04/06/1028_1.png">
</pre>

<pre>
<strong>Example 3:</strong>
<strong>Input:</strong> [1,1,0,1,1,0,1,0]
<strong>Output:</strong> [1,1,0,1,1,null,1]
<img src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/04/05/1028.png">
</pre>

#### Note:
* The binary tree will have at most ```100 nodes```.
* The value of each node will only be ```0``` or ```1```.

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
    def pruneTree(self, root: TreeNode) -> TreeNode:
        if not root:
            return None

        root.left = self.pruneTree(root.left)
        root.right = self.pruneTree(root.right)

        if root.val == 1 or root.left or root.right:
            return root
        else:
            return None
```
