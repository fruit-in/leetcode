# 2265. Count Nodes Equal to Average of Subtree
Given the `root` of a binary tree, return *the number of nodes where the value of the node is equal to the **average** of the values in its **subtree***.

#### Note:
* The **average** of `n` elements is the **sum** of the `n` elements divided by `n` and **rounded down** to the nearest integer.
* A **subtree** of `root` is a tree consisting of `root` and all of its descendants.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/03/15/image-20220315203925-1.png)
<pre>
<strong>Input:</strong> root = [4,8,5,0,1,null,6]
<strong>Output:</strong> 5
<strong>Explanation:</strong>
For the node with value 4: The average of its subtree is (4 + 8 + 5 + 0 + 1 + 6) / 6 = 24 / 6 = 4.
For the node with value 5: The average of its subtree is (5 + 6) / 2 = 11 / 2 = 5.
For the node with value 0: The average of its subtree is 0 / 1 = 0.
For the node with value 1: The average of its subtree is 1 / 1 = 1.
For the node with value 6: The average of its subtree is 6 / 1 = 6.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/03/26/image-20220326133920-1.png)
<pre>
<strong>Input:</strong> root = [1]
<strong>Output:</strong> 1
<strong>Explanation:</strong> For the node with value 1: The average of its subtree is 1 / 1 = 1.
</pre>

#### Constraints:
* The number of nodes in the tree is in the range `[1, 1000]`.
* `0 <= Node.val <= 1000`

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
    def averageOfSubtree(self, root: Optional[TreeNode]) -> int:
        return self.dfs(root)[2]

    def dfs(self, root: TreeNode) -> (int, int, int):
        if not root:
            return (0, 0, 0)

        count_left, sum_left, ret_left = self.dfs(root.left)
        count_right, sum_right, ret_right = self.dfs(root.right)
        count_root = count_left + count_right + 1
        sum_root = sum_left + sum_right + root.val
        ret_root = ret_left + ret_right + \
            (1 if root.val == sum_root // count_root else 0)

        return (count_root, sum_root, ret_root)
```
