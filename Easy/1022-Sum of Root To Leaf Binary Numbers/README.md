# 1022. Sum of Root To Leaf Binary Numbers
Given a binary tree, each node has value ```0``` or ```1```.  Each root-to-leaf path represents a binary number starting with the most significant bit.  For example, if the path is ```0 -> 1 -> 1 -> 0 -> 1```, then this could represent ```01101``` in binary, which is ```13```.

For all leaves in the tree, consider the numbers represented by the path from the root to that leaf.

Return the sum of these numbers.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/04/04/sum-of-root-to-leaf-binary-numbers.png)
<pre>
<strong>Input:</strong> [1,0,1,0,1,0,1]
<strong>Output:</strong> 22
<strong>Explanation:</strong> (100) + (101) + (110) + (111) = 4 + 5 + 6 + 7 = 22
</pre>

#### Note:
1. The number of nodes in the tree is between ```1``` and ```1000```.
2. node.val is ```0``` or ```1```.
3. The answer will not exceed ```2^31 - 1```.

## Solutions (Python)

### 1. Recursion
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def sumRootToLeaf(self, root: TreeNode) -> int:
        def helper(root: TreeNode, n: int) -> int:
            n = (n << 1) + root.val

            if root.left and root.right:
                return helper(root.left, n) + helper(root.right, n)
            elif root.left:
                return helper(root.left, n)
            elif root.right:
                return helper(root.right, n)
            else:
                return n

        return helper(root, 0)
```

### 2. Iteration
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def sumRootToLeaf(self, root: TreeNode) -> int:
        nodes = [(root, 0)]
        nums = []

        while nodes:
            cur, num = nodes.pop()

            if not cur.left and not cur.right:
                nums.append((num << 1) + cur.val)
            if cur.left:
                nodes.append((cur.left, (num << 1) + cur.val))
            if cur.right:
                nodes.append((cur.right, (num << 1) + cur.val))

        return sum(nums)
```
