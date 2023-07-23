# 671. Second Minimum Node In a Binary Tree
Given a non-empty special binary tree consisting of nodes with the non-negative value, where each node in this tree has exactly ```two``` or ```zero``` sub-node. If the node has two sub-nodes, then this node's value is the smaller value among its two sub-nodes. More formally, the property ```root.val = min(root.left.val, root.right.val)``` always holds.

Given such a binary tree, you need to output the **second minimum** value in the set made of all the nodes' value in the whole tree.

If no such second minimum value exists, output -1 instead.

#### Example 1:
<pre>
<strong>Input:</strong>
    2
   / \
  2   5
     / \
    5   7
<strong>Output:</strong> 5
<strong>Explanation:</strong> The smallest value is 2, the second smallest value is 5.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
    2
   / \
  2   2
<strong>Output:</strong> -1
<strong>Explanation:</strong> The smallest value is 2, but there isn't any second smallest value.
</pre>

## Solutions (Python)

### 1. Brute Force
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def findSecondMinimumValue(self, root: TreeNode) -> int:
        vals = set()
        nodes = [root]

        while nodes:
            curr = nodes.pop()
            vals.add(curr.val)
            if curr.left:
                nodes.append(curr.left)
                nodes.append(curr.right)

        vals.remove(min(vals))

        return min(vals) if vals else -1
```

### 2. Store 1st & 2nd Minimum Values
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def findSecondMinimumValue(self, root: TreeNode) -> int:
        ret = float('+inf')
        nodes = [root]

        while nodes:
            curr = nodes.pop()
            if root.val < curr.val < ret:
                ret = curr.val
            elif curr.val == root.val and curr.left:
                nodes.append(curr.left)
                nodes.append(curr.right)

        return ret if ret < float('+inf') else -1
```
