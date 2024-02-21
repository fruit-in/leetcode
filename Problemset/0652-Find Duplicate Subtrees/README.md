# 652. Find Duplicate Subtrees
Given the `root` of a binary tree, return all **duplicate subtrees**.

For each kind of duplicate subtrees, you only need to return the root node of any **one** of them.

Two trees are **duplicate** if they have the **same structure** with the **same node values**.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/08/16/e1.jpg)
<pre>
<strong>Input:</strong> root = [1,2,3,4,null,2,4,null,null,4]
<strong>Output:</strong> [[2,4],[4]]
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/08/16/e2.jpg)
<pre>
<strong>Input:</strong> root = [2,1,1]
<strong>Output:</strong> [[1]]
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2020/08/16/e33.jpg)
<pre>
<strong>Input:</strong> root = [2,2,2,3,null,3,null]
<strong>Output:</strong> [[2,3],[3]]
</pre>

#### Constraints:
* The number of the nodes in the tree will be in the range `[1, 5000]`
* `-200 <= Node.val <= 200`

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
    def findDuplicateSubtrees(self, root: Optional[TreeNode]) -> List[Optional[TreeNode]]:
        count = {}
        ret = []

        def dfs(root: Optional[TreeNode]) -> str:
            if root is None:
                return "()"

            s = "{}({})({})".format(root.val, dfs(root.left), dfs(root.right))
            count[s] = count.get(s, 0) + 1

            if count[s] == 2:
                ret.append(root)

            return s

        dfs(root)

        return ret
```
