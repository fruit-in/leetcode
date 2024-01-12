# 1339. Maximum Product of Splitted Binary Tree
Given the `root` of a binary tree, split the binary tree into two subtrees by removing one edge such that the product of the sums of the subtrees is maximized.

Return *the maximum product of the sums of the two subtrees*. Since the answer may be too large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

**Note** that you need to maximize the answer before taking the mod and not after taking it.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/01/21/sample_1_1699.png)
<pre>
<strong>Input:</strong> root = [1,2,3,4,5,6]
<strong>Output:</strong> 110
<strong>Explanation:</strong> Remove the red edge and get 2 binary trees with sum 11 and 10. Their product is 110 (11*10)
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/01/21/sample_2_1699.png)
<pre>
<strong>Input:</strong> root = [1,null,2,3,4,null,null,5,6]
<strong>Output:</strong> 90
<strong>Explanation:</strong> Remove the red edge and get 2 binary trees with sum 15 and 6.Their product is 90 (15*6)
</pre>

#### Constraints:
* The number of nodes in the tree is in the range <code>[2, 5 * 10<sup>4</sup>]</code>.
* <code>1 <= Node.val <= 10<sup>4</sup></code>

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
    def maxProduct(self, root: Optional[TreeNode]) -> int:
        def subtreeSum(root: Optional[TreeNode]) -> int:
            if root is None:
                return 0

            root.val += subtreeSum(root.left)
            root.val += subtreeSum(root.right)

            return root.val

        subtreeSum(root)

        nodes = [root]
        ret = 0

        while nodes != []:
            curr = nodes.pop()
            ret = max(ret, curr.val * (root.val - curr.val))

            if curr.left is not None:
                nodes.append(curr.left)
            if curr.right is not None:
                nodes.append(curr.right)

        return ret % 1000000007
```
