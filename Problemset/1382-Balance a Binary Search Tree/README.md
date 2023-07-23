# 1382. Balance a Binary Search Tree
Given a binary search tree, return a **balanced** binary search tree with the same node values.

A binary search tree is *balanced* if and only if the depth of the two subtrees of every node never differ by more than 1.

If there is more than one answer, return any of them.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/08/22/1515_ex1.png)![](https://assets.leetcode.com/uploads/2019/08/22/1515_ex1_out.png)
<pre>
<b>Input:</b> root = [1,null,2,null,3,null,4,null,null]
<b>Output:</b> [2,1,3,null,null,null,4]
<b>Explanation:</b> This is not the only correct answer, [3,1,4,null,2,null,null] is also correct.
</pre>

#### Constraints:
* The number of nodes in the tree is between `1` and `10^4`.
* The tree nodes will have distinct values between `1` and `10^5`.

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
    def balanceBST(self, root: TreeNode) -> TreeNode:
        def foo(vals: List[int]) -> TreeNode:
            if not vals:
                return None

            mid = len(vals) // 2

            return TreeNode(vals[mid], foo(vals[:mid]), foo(vals[mid + 1:]))

        curr = root
        stack = []
        vals = []

        while stack or curr:
            while curr:
                stack.append(curr)
                curr = curr.left

            curr = stack.pop()
            vals.append(curr.val)

            curr = curr.right

        return foo(vals)
```
