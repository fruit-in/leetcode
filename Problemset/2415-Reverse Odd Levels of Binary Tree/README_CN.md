# 2415. 反转二叉树的奇数层
给你一棵 **完美** 二叉树的根节点 `root` ，请你反转这棵树中每个 **奇数** 层的节点值。

* 例如，假设第 3 层的节点值是 `[2,1,3,4,7,11,29,18]` ，那么反转后它应该变成 `[18,29,11,7,4,3,1,2]` 。

反转后，返回树的根节点。

**完美** 二叉树需满足：二叉树的所有父节点都有两个子节点，且所有叶子节点都在同一层。

节点的 **层数** 等于该节点到根节点之间的边数。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/07/28/first_case1.png)
<pre>
<strong>输入:</strong> root = [2,3,5,8,13,21,34]
<strong>输出:</strong> [2,5,3,8,13,21,34]
<strong>解释:</strong>
这棵树只有一个奇数层。
在第 1 层的节点分别是 3、5 ，反转后为 5、3 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2022/07/28/second_case3.png)
<pre>
<strong>输入:</strong> root = [7,13,11]
<strong>输出:</strong> [7,11,13]
<strong>解释:</strong>
在第 1 层的节点分别是 13、11 ，反转后为 11、13 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> root = [0,1,2,0,0,0,0,1,1,1,1,2,2,2,2]
<strong>输出:</strong> [0,2,1,0,0,0,0,2,2,2,2,1,1,1,1]
<strong>解释:</strong>
奇数层由非零值组成。
在第 1 层的节点分别是 1、2 ，反转后为 2、1 。
在第 3 层的节点分别是 1、1、1、1、2、2、2、2 ，反转后为 2、2、2、2、1、1、1、1 。
</pre>

#### 提示:
* 树中的节点数目在范围 <code>[1, 2<sup>14</sup>]</code> 内
* <code>0 <= Node.val <= 10<sup>5</sup></code>
* `root` 是一棵 **完美** 二叉树

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
    def reverseOddLevels(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        isOdd = False
        currLevel = [root]

        while currLevel:
            nextLevel = []
            if currLevel[0].left is not None:
                for node in currLevel:
                    nextLevel.append(node.left)
                    nextLevel.append(node.right)

            isOdd = not isOdd
            currLevel = nextLevel

            if isOdd:
                currVals = [node.val for node in currLevel[::-1]]
                for i in range(len(currLevel)):
                    currLevel[i].val = currVals[i]

        return root
```
