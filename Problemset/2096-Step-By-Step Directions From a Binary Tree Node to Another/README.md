# 2096. Step-By-Step Directions From a Binary Tree Node to Another
You are given the `root` of a **binary tree** with `n` nodes. Each node is uniquely assigned a value from `1` to `n`. You are also given an integer `startValue` representing the value of the start node `s`, and a different integer `destValue` representing the value of the destination node `t`.

Find the **shortest path** starting from node `s` and ending at node `t`. Generate step-by-step directions of such path as a string consisting of only the **uppercase** letters `'L'`, `'R'`, and `'U'`. Each letter indicates a specific direction:

* `'L'` means to go from a node to its **left child** node.
* `'R'` means to go from a node to its **right child** node.
* `'U'` means to go from a node to its **parent** node.

Return *the step-by-step directions of the **shortest path** from node* `s` *to node* `t`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/11/15/eg1.png)
<pre>
<strong>Input:</strong> root = [5,1,2,3,null,6,4], startValue = 3, destValue = 6
<strong>Output:</strong> "UURL"
<strong>Explanation:</strong> The shortest path is: 3 → 1 → 5 → 2 → 6.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/11/15/eg2.png)
<pre>
<strong>Input:</strong> root = [2,1], startValue = 2, destValue = 1
<strong>Output:</strong> "L"
<strong>Explanation:</strong> The shortest path is: 2 → 1.
</pre>

#### Constraints:
* The number of nodes in the tree is `n`.
* <code>2 <= n <= 10<sup>5</sup></code>
* `1 <= Node.val <= n`
* All the values in the tree are **unique**.
* `1 <= startValue, destValue <= n`
* `startValue != destValue`

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
    def getDirections(self, root: Optional[TreeNode], startValue: int, destValue: int) -> str:
        root2Start = self.root2(root, startValue)
        root2Dest = self.root2(root, destValue)
        minPathLen = min(len(root2Start), len(root2Dest))

        for i in range(minPathLen + 1):
            if i == minPathLen or root2Start[i] != root2Dest[i]:
                return 'U' * (len(root2Start) - i) + root2Dest[i:]

    def root2(self, root: Optional[TreeNode], destValue: int) -> Optional[str]:
        if root is None:
            return None

        if root.val == destValue:
            return ""

        root2Left = self.root2(root.left, destValue)
        if root2Left is not None:
            return 'L' + root2Left

        root2Right = self.root2(root.right, destValue)
        if root2Right is not None:
            return 'R' + root2Right

        return None
```
