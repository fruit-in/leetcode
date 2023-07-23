# 429. N-ary Tree Level Order Traversal
Given an n-ary tree, return the level order traversal of its nodes' values. (ie, from left to right, level by level).

For example, given a <code>3-ary</code> tree:
![](https://assets.leetcode.com/uploads/2018/10/12/narytreeexample.png)

We should return its level order traversal:
<pre>
[
     [1],
     [3,2,4],
     [5,6]
]
</pre>

#### Note:
1. The depth of the tree is at most <code>1000</code>.
2. The total number of nodes is at most <code>5000</code>.

## Solutions (Python)

### 1. BFS
```Python3
"""
# Definition for a Node.
class Node:
    def __init__(self, val, children):
        self.val = val
        self.children = children
"""
class Solution:
    def levelOrder(self, root: 'Node') -> List[List[int]]:
        if not root:
            return []
        ret = []
        nodes = [root]
        while nodes:
            level = []
            temp = []
            for cur in nodes:
                level.append(cur.val)
                temp.extend(cur.children)
            nodes = temp
            ret.append(level)
        return ret
```

## Solutions (Ruby)

### 1. BFS
```Ruby
# Definition for a Node.
# class Node
#     attr_accessor :val, :children
#     def initialize(val)
#         @val = val
#         @children = []
#     end
# end

# @param {Node} root
# @return {List[List[int]]}
def levelOrder(root)
    return [] if root.nil?

    ret = []
    nodes = [root]

    while not nodes.empty?
        level = []
        temp = []
        for curr in nodes
            level.push(curr.val)
            temp.concat(curr.children)
        end
        nodes = temp
        ret.push(level)
    end

    return ret
end
```
