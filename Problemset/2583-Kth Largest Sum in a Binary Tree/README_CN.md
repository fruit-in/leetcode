# 2583. 二叉树中的第 K 大层和
给你一棵二叉树的根节点 `root` 和一个正整数 `k` 。

树中的 **层和** 是指 **同一层** 上节点值的总和。

返回树中第 `k` 大的层和（不一定不同）。如果树少于 `k` 层，则返回 `-1` 。

**注意**，如果两个节点与根节点的距离相同，则认为它们在同一层。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/12/14/binaryytreeedrawio-2.png)
<pre>
<strong>输入:</strong> root = [5,8,9,2,1,3,7,4,6], k = 2
<strong>输出:</strong> 13
<strong>解释:</strong> 树中每一层的层和分别是：
- Level 1: 5
- Level 2: 8 + 9 = 17
- Level 3: 2 + 1 + 3 + 7 = 13
- Level 4: 4 + 6 = 10
第 2 大的层和等于 13 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/12/14/treedrawio-3.png)
<pre>
<strong>输入:</strong> root = [1,2,null,3], k = 1
<strong>输出:</strong> 3
<strong>解释:</strong> 最大的层和是 3 。
</pre>

#### 提示:
* 树中的节点数为 `n`
* <code>2 <= n <= 10<sup>5</sup></code>
* <code>1 <= Node.val <= 10<sup>6</sup></code>
* `1 <= k <= n`

## 题解 (Python)

### 1. 题解
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
