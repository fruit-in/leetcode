# 559. Maximum Depth of N-ary Tree
Given a n-ary tree, find its maximum depth.

The maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.

For example, given a <code>3-ary</code> tree:
![](https://assets.leetcode.com/uploads/2018/10/12/narytreeexample.png)<br>
We should return its max depth, which is 3.

#### Note:
1. The depth of the tree is at most <code>1000</code>.
2. The total number of nodes is at most <code>5000</code>.

## Solutions (Python)

### 1. DFS
```Python3
"""
# Definition for a Node.
class Node:
    def __init__(self, val, children):
        self.val = val
        self.children = children
"""

class Solution:
    def maxDepth(self, root: 'Node') -> int:
        if not root:
            return 0
        if not root.children:
            return 1
        return max(map(self.maxDepth, root.children)) + 1
```

### 2. BFS
```Python3
"""
# Definition for a Node.
class Node:
    def __init__(self, val, children):
        self.val = val
        self.children = children
"""

class Solution:
    def maxDepth(self, root: 'Node') -> int:
        if not root:
            return 0
        depth = 1
        nodes = [n for n in root.children]
        while nodes:
            depth += 1
            nodes = [n for node in nodes for n in node.children]
        return depth
```
