# 589. N-ary Tree Preorder Traversal
Given an n-ary tree, return the *preorder* traversal of its nodes' values.

For example, given a <code>3-ary</code> tree:
![](https://assets.leetcode.com/uploads/2018/10/12/narytreeexample.png)

Return its preorder traversal as: <code>[1,3,5,6,2,4]</code>.

#### Note:
Recursive solution is trivial, could you do it iteratively?

## Solutions (Python)

### 1. Recursion
```Python3
"""
# Definition for a Node.
class Node:
    def __init__(self, val, children):
        self.val = val
        self.children = children
"""
class Solution:
    def preorder(self, root: 'Node') -> List[int]:
        if not root:
            return []
        vals = [root.val]
        for child in root.children:
            vals.extend(self.preorder(child))
        return vals
```

### 2. Iteration
```Python3
"""
# Definition for a Node.
class Node:
    def __init__(self, val, children):
        self.val = val
        self.children = children
"""
class Solution:
    def preorder(self, root: 'Node') -> List[int]:
        vals = []
        nodes = [root]
        while nodes:
            cur = nodes.pop()
            if cur:
                vals.append(cur.val)
                for n in cur.children[::-1]:
                    nodes.append(n)
        return vals
```
