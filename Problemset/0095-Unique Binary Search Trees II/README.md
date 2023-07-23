# 95. Unique Binary Search Trees II
Given an integer `n`, generate all structurally unique **BST's** (binary search trees) that store values 1 ... *n*.

#### Example:
<pre>
<strong>Input:</strong> 3
<strong>Output:</strong>
[
  [1,null,3,2],
  [3,2,null,1],
  [3,1,null,null,2],
  [2,1,3],
  [1,null,2,null,3]
]
<strong>Explanation:</strong>
The above output corresponds to the 5 unique BST's shown below:

   1         3     3      2      1
    \       /     /      / \      \
     3     2     1      1   3      2
    /     /       \                 \
   2     1         2                 3
</pre>

#### Constraints:
* `0 <= n <= 8`

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
    def generateTrees(self, n: int) -> List[TreeNode]:
        def helper(m: int, n: int) -> List[TreeNode]:
            if m > n:
                return [None]

            ret = []

            for val in range(m, n + 1):
                for left in helper(m, val - 1):
                    for right in helper(val + 1, n):
                        ret.append(TreeNode(val, left, right))

            return ret

        return helper(1, n) if n > 0 else []
```
