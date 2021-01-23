# 1008. Construct Binary Search Tree from Preorder Traversal
Return the root node of a binary **search** tree that matches the given `preorder` traversal.

*(Recall that a binary search tree is a binary tree where for every node, any descendant of `node.left` has a value `<` `node.val`, and any descendant of `node.right` has a value `>` `node.val`.  Also recall that a preorder traversal displays the value of the `node` first, then traverses `node.left`, then traverses `node.right`.)*

It's guaranteed that for the given test cases there is always possible to find a binary search tree with the given requirements.

#### Example 1:
<pre>
<strong>Input:</strong> [8,5,1,7,10,12]
<strong>Output:</strong> [8,5,10,1,7,null,12]
<img src="https://assets.leetcode.com/uploads/2019/03/06/1266.png">
</pre>

#### Constraints:
* `1 <= preorder.length <= 100`
* `1 <= preorder[i] <= 10^8`
* The values of `preorder` are distinct.

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
    def bstFromPreorder(self, preorder: List[int]) -> TreeNode:
        if not preorder:
            return None

        return TreeNode(
            preorder[0],
            self.bstFromPreorder(
                [val for val in preorder if val < preorder[0]]),
            self.bstFromPreorder(
                [val for val in preorder if val > preorder[0]])
        )
```
