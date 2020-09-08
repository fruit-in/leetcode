# 1457. Pseudo-Palindromic Paths in a Binary Tree
Given a binary tree where node values are digits from 1 to 9. A path in the binary tree is said to be **pseudo-palindromic** if at least one permutation of the node values in the path is a palindrome.

*Return the number of **pseudo-palindromic** paths going from the root node to leaf nodes.*

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/05/06/palindromic_paths_1.png)
<pre>
<strong>Input:</strong> root = [2,3,1,3,1,null,1]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The figure above represents the given binary tree. There are three paths going from the root node to leaf nodes: the red path [2,3,3], the green path [2,1,1], and the path [2,3,1]. Among these paths only red path and green path are pseudo-palindromic paths since the red path [2,3,3] can be rearranged in [3,2,3] (palindrome) and the green path [2,1,1] can be rearranged in [1,2,1] (palindrome).
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/05/07/palindromic_paths_2.png)
<pre>
<strong>Input:</strong> root = [2,1,1,1,3,null,null,null,null,null,1]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The figure above represents the given binary tree. There are three paths going from the root node to leaf nodes: the green path [2,1,1], the path [2,1,3,1], and the path [2,1]. Among these paths only the green path is pseudo-palindromic since [2,1,1] can be rearranged in [1,2,1] (palindrome).
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> root = [9]
<strong>Output:</strong> 1
</pre>

#### Constraints:
* The given binary tree will have between `1` and `10^5` nodes.
* Node values are digits from `1` to `9`.

## Solutions (Python)

### 1. BFS
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def pseudoPalindromicPaths(self, root: TreeNode) -> int:
        def foo(root: TreeNode, x: int) -> int:
            x ^= 1 << root.val

            if not root.left and not root.right:
                return 1 if bin(x).count('1') < 2 else 0
            elif not root.left:
                return foo(root.right, x)
            elif not root.right:
                return foo(root.left, x)
            else:
                return foo(root.left, x) + foo(root.right, x)

        return foo(root, 0)
```
