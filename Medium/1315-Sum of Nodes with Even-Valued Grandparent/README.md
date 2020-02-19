# 1315. Sum of Nodes with Even-Valued Grandparent
Given a binary tree, return the sum of values of nodes with even-valued grandparent.  (A *grandparent* of a node is the parent of its parent, if it exists.)

If there are no nodes with an even-valued grandparent, return ```0```.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/07/24/1473_ex1.png)
<pre>
<strong>Input:</strong> root = [6,7,8,2,7,1,3,9,null,1,4,null,null,null,5]
<strong>Output:</strong> 18
<strong>Explanation:</strong> The red nodes are the nodes with even-value grandparent while the blue nodes are the even-value grandparents.
</pre>

#### Constraints:
* The number of nodes in the tree is between ```1``` and ```10^4```.
* The value of nodes is between ```1``` and ```100```.

## Solutions (Python)

### 1. DFS
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def sumEvenGrandparent(self, root: TreeNode) -> int:
        stack = [root]
        ret = 0

        while stack:
            curr = stack.pop()

            if curr.left:
                stack.append(curr.left)
                if curr.val % 2 == 0:
                    ret += curr.left.left.val if curr.left.left else 0
                    ret += curr.left.right.val if curr.left.right else 0
            if curr.right:
                stack.append(curr.right)
                if curr.val % 2 == 0:
                    ret += curr.right.left.val if curr.right.left else 0
                    ret += curr.right.right.val if curr.right.right else 0

        return ret
```
