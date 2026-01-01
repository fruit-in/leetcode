# 99. Recover Binary Search Tree
You are given the `root` of a binary search tree (BST), where the values of **exactly** two nodes of the tree were swapped by mistake. *Recover the tree without changing its structure*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/10/28/recover1.jpg)
<pre>
<strong>Input:</strong> root = [1,3,null,null,2]
<strong>Output:</strong> [3,1,null,null,2]
<strong>Explanation:</strong> 3 cannot be a left child of 1 because 3 > 1. Swapping 1 and 3 makes the BST valid.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/10/28/recover2.jpg)
<pre>
<strong>Input:</strong> root = [3,1,4,null,null,2]
<strong>Output:</strong> [2,1,4,null,null,3]
<strong>Explanation:</strong> 2 cannot be in the right subtree of 3 because 2 < 3. Swapping 2 and 3 makes the BST valid.
</pre>

#### Constraints:
* The number of nodes in the tree is in the range `[2, 1000]`.
* <code>-2<sup>31</sup> <= Node.val <= 2<sup>31</sup> - 1</code>

**Follow up:** A solution using `O(n)` space is pretty straight-forward. Could you devise a constant `O(1)` space solution?

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
    def recoverTree(self, root: Optional[TreeNode]) -> None:
        """
        Do not return anything, modify root in-place instead.
        """
        curr = root
        node1, node2 = None, TreeNode(-inf)
        wrong1, wrong2 = None, None

        while curr:
            if not curr.left:
                node1, node2 = node2, curr
                if not wrong1 and node1.val > node2.val:
                    wrong1 = node1
                if node1.val > node2.val:
                    wrong2 = node2
                curr = curr.right
            else:
                temp = curr.left
                while temp.right and temp.right != curr:
                    temp = temp.right
                if not temp.right:
                    temp.right = curr
                    curr = curr.left
                else:
                    node1, node2 = node2, curr
                    if not wrong1 and node1.val > node2.val:
                        wrong1 = node1
                    if node1.val > node2.val:
                        wrong2 = node2
                    temp.right = None
                    curr = curr.right

        wrong1.val, wrong2.val = wrong2.val, wrong1.val
```
