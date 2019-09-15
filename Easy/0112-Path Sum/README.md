# 112. Path Sum
Given a binary tree and a sum, determine if the tree has a root-to-leaf path such that adding up all the values along the path equals the given sum.

**Note:** A leaf is a node with no children.

#### Example:
Given the below binary tree and ```sum = 22```,
<pre>
      <strong>5</strong>
     / \
    <strong>4</strong>   8
   /   / \
  <strong>11</strong>  13  4
 /  \      \
7    <strong>2</strong>      1
</pre>
return true, as there exist a root-to-leaf path ```5->4->11->2``` which sum is 22.

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
    def hasPathSum(self, root: TreeNode, sum: int) -> bool:
        if not root:
            return False
        sum -= root.val
        if not root.left and not root.right:
            return sum == 0
        return self.hasPathSum(root.left, sum) or self.hasPathSum(root.right, sum)
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
    def hasPathSum(self, root: TreeNode, sum: int) -> bool:
        if not root:
            return False
        root.val = sum - root.val
        nodes = [root]
        while nodes:
            curr = nodes.pop()
            if curr.val == 0 and not curr.left and not curr.right:
                return True
            if curr.left:
                curr.left.val = curr.val - curr.left.val
                nodes.append(curr.left)
            if curr.right:
                curr.right.val = curr.val - curr.right.val
                nodes.append(curr.right)
        return False
```
