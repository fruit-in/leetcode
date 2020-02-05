# 1026. Maximum Difference Between Node and Ancestor
Given the ```root``` of a binary tree, find the maximum value ```V``` for which there exists **different** nodes ```A``` and ```B``` where ```V = |A.val - B.val|``` and ```A``` is an ancestor of ```B```.

(A node A is an ancestor of B if either: any child of A is equal to B, or any child of A is an ancestor of B.)

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/09/09/2whqcep.jpg)
<pre>
<strong>Input:</strong> [8,3,10,1,6,null,14,null,null,4,7,13]
<strong>Output:</strong> 7
<strong>Explanation:</strong>
We have various ancestor-node differences, some of which are given below :
|8 - 3| = 5
|3 - 7| = 4
|8 - 1| = 7
|10 - 13| = 3
Among all possible differences, the maximum value of 7 is obtained by |8 - 1| = 7.
</pre>

#### Note:
1. The number of nodes in the tree is between ```2``` and ```5000```.
2. Each node will have value between ```0``` and ```100000```.

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
    def maxAncestorDiff(self, root: TreeNode) -> int:
        def helper(root: TreeNode) -> (int, int, int):
            if not root:
                return (100001, -1, -1)

            l_min, l_max, l_diff = helper(root.left)
            r_min, r_max, r_diff = helper(root.right)

            lr_min = min(l_min, r_min)
            lr_max = max(l_max, r_max)

            diff = max(l_diff, r_diff)
            if lr_min != 100001:
                diff = max(diff, abs(root.val - lr_min))
            if lr_max != -1:
                diff = max(diff, abs(root.val - lr_max))

            return (min(root.val, lr_min), max(root.val, lr_max), diff)

        return helper(root)[2]
```
