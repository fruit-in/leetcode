# 590. N-ary Tree Postorder Traversal
Given an n-ary tree, return the *postorder* traversal of its nodes' values.

For example, given a <code>3-ary</code> tree:
![](https://assets.leetcode.com/uploads/2018/10/12/narytreeexample.png)

Return its postorder traversal as: <code>[5,6,3,2,4,1]</code>.

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
    def postorder(self, root: 'Node') -> List[int]:
        if not root:
            return []
        vals = []
        for n in root.children:
            vals.extend(self.postorder(n))
        vals.append(root.val)
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
    def postorder(self, root: 'Node') -> List[int]:
        if not root:
            return []
        nset = set()
        vals = []
        nodes = [root]
        while nodes:
            cur = nodes.pop()
            if not cur.children or cur.children[0] in nset:
                vals.append(cur.val)
                nset.add(cur)
            else:
                nodes.append(cur)
                for child in cur.children[::-1]:
                    nodes.append(child)
        return vals
```
