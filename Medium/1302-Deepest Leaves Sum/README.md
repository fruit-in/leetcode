# 1302. Deepest Leaves Sum
Given a binary tree, return the sum of values of its deepest leaves.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/07/31/1483_ex1.png)
<pre>
<strong>Input:</strong> root = [1,2,3,4,5,null,6,7,null,null,null,null,8]
<strong>Output:</strong> 15
</pre>

#### Constraints:
* The number of nodes in the tree is between ```1``` and ```10^4```.
* The value of nodes is between ```1``` and ```100```.

## Solutions (Python)

### 1. Level Order Traversal
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def deepestLeavesSum(self, root: TreeNode) -> int:
        curr = [root]

        while True:
            next = [n.left for n in curr if n.left]
            next.extend(n.right for n in curr if n.right)

            if not next:
                return sum(n.val for n in curr)

            curr = next
```
