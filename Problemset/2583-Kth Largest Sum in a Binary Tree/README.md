# 2583. Kth Largest Sum in a Binary Tree
You are given the `root` of a binary tree and a positive integer `k`.

The **level sum** in the tree is the sum of the values of the nodes that are on the **same** level.

Return *the* <code>k<sup>th</sup></code> ***largest** level sum in the tree (not necessarily distinct)*. If there are fewer than `k` levels in the tree, return `-1`.

**Note** that two nodes are on the same level if they have the same distance from the root.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/12/14/binaryytreeedrawio-2.png)
<pre>
<strong>Input:</strong> root = [5,8,9,2,1,3,7,4,6], k = 2
<strong>Output:</strong> 13
<strong>Explanation:</strong> The level sums are the following:
- Level 1: 5.
- Level 2: 8 + 9 = 17.
- Level 3: 2 + 1 + 3 + 7 = 13.
- Level 4: 4 + 6 = 10.
The 2nd largest level sum is 13.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2022/12/14/treedrawio-3.png)
<pre>
<strong>Input:</strong> root = [1,2,null,3], k = 1
<strong>Output:</strong> 3
<strong>Explanation:</strong> The largest level sum is 3.
</pre>

#### Constraints:
* The number of nodes in the tree is `n`.
* <code>2 <= n <= 10<sup>5</sup></code>
* <code>1 <= Node.val <= 10<sup>6</sup></code>
* `1 <= k <= n`

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
    def kthLargestLevelSum(self, root: Optional[TreeNode], k: int) -> int:
        def dfs(root: Optional[TreeNode], depth: int) -> None:
            if root is not None:
                if len(levelsum) < depth:
                    levelsum.append(0)
                levelsum[depth - 1] += root.val
                dfs(root.left, depth + 1)
                dfs(root.right, depth + 1)

        levelsum = []
        dfs(root, 1)

        return sorted(levelsum)[-k] if k <= len(levelsum) else -1
```
