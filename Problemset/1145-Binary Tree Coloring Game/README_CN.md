# 1145. 二叉树着色游戏
有两位极客玩家参与了一场「二叉树着色」的游戏。游戏中，给出二叉树的根节点 `root`，树上总共有 `n` 个节点，且 `n` 为奇数，其中每个节点上的值从 `1` 到 `n` 各不相同。

最开始时：

* 「一号」玩家从 `[1, n]` 中取一个值 `x`（`1 <= x <= n`）；
* 「二号」玩家也从 `[1, n]` 中取一个值 `y`（`1 <= y <= n`）且 `y != x`。

「一号」玩家给值为 `x` 的节点染上红色，而「二号」玩家给值为 `y` 的节点染上蓝色。

之后两位玩家轮流进行操作，「一号」玩家先手。每一回合，玩家选择一个被他染过色的节点，将所选节点一个 **未着色** 的邻节点（即左右子节点、或父节点）进行染色（「一号」玩家染红色，「二号」玩家染蓝色）。

如果（且仅在此种情况下）当前玩家无法找到这样的节点来染色时，其回合就会被跳过。

若两个玩家都没有可以染色的节点时，游戏结束。着色节点最多的那位玩家获得胜利。

现在，假设你是「二号」玩家，根据所给出的输入，假如存在一个 `y` 值可以确保你赢得这场游戏，则返回 `true` ；若无法获胜，就请返回 `false` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2019/08/01/1480-binary-tree-coloring-game.png)
<pre>
<strong>输入:</strong> root = [1,2,3,4,5,6,7,8,9,10,11], n = 11, x = 3
<strong>输出:</strong> true
<strong>解释:</strong> 第二个玩家可以选择值为 2 的节点。
</pre>

#### 示例 1:
<pre>
<strong>输入:</strong> root = [1,2,3], n = 3, x = 1
<strong>输出:</strong> false
</pre>

#### 提示:
* 树中节点数目为 `n`
* `1 <= x <= n <= 100`
* `n` 是奇数
* `1 <= Node.val <= n`
* 树中所有值 **互不相同**

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
    def btreeGameWinningMove(self, root: Optional[TreeNode], n: int, x: int) -> bool:
        nodes = [root]
        nodex = root

        while nodes != []:
            curr = nodes.pop()

            if curr.left is not None:
                if curr.left.val == x:
                    nodex = curr.left
                    break
                nodes.append(curr.left)
            if curr.right is not None:
                if curr.right.val == x:
                    nodex = curr.right
                    break
                nodes.append(curr.right)

        for node in [root, nodex.left, nodex.right]:
            if node is None or node.val == x:
                continue

            nodes = [node]
            count = 0

            while nodes != []:
                curr = nodes.pop()
                count += 1

                if curr.left is not None and curr.left.val != x:
                    nodes.append(curr.left)
                if curr.right is not None and curr.right.val != x:
                    nodes.append(curr.right)

            if count > n // 2:
                return True

        return False
```
