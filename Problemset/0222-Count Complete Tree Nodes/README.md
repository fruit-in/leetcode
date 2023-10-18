# 222. Count Complete Tree Nodes
Given the `root` of a **complete** binary tree, return the number of the nodes in the tree.

According to [**Wikipedia**](http://en.wikipedia.org/wiki/Binary_tree#Types_of_binary_trees), every level, except possibly the last, is completely filled in a complete binary tree, and all nodes in the last level are as far left as possible. It can have between `1` and <code>2<sup>h</sup></code> nodes inclusive at the last level `h`.

Design an algorithm that runs in less than `O(n)` time complexity.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/01/14/complete.jpg)
<pre>
<strong>Input:</strong> root = [1,2,3,4,5,6]
<strong>Output:</strong> 6
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> root = []
<strong>Output:</strong> 0
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> root = [1]
<strong>Output:</strong> 1
</pre>

#### Constraints:
* The number of nodes in the tree is in the range <code>[0, 5 * 10<sup>4</sup>]</code>.
* <code>0 <= Node.val <= 5 * 10<sup>4</sup></code>
* The tree is guaranteed to be **complete**.

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
    def countNodes(self, root: Optional[TreeNode]) -> int:
        low = 0
        high = 1
        curr = root
        root = TreeNode(left=TreeNode(), right=root)

        while curr is not None:
            low = (low << 1) + 1
            high = (high << 1) + 1
            curr = curr.right

        while low < high:
            mid = (low + high) // 2
            curr = root
            flag = True

            for bit in bin(mid)[2:]:
                if bit == '0':
                    curr = curr.left
                else:
                    curr = curr.right

                if curr is None:
                    flag = False
                    break

            if flag:
                low = mid + 1
            else:
                high = mid

        return low - 1
```
