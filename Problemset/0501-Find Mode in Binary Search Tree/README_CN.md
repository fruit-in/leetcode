# 501. 二叉搜索树中的众数
给定一个有相同值的二叉搜索树（BST），找出 BST 中的所有众数（出现频率最高的元素）。

假定 BST 有如下定义：
* 结点左子树中所含结点的值小于等于当前结点的值
* 结点右子树中所含结点的值大于等于当前结点的值
* 左子树和右子树都是二叉搜索树

例如：<br>
给定 BST ```[1,null,2,2]```,
```
   1
    \
     2
    /
   2
```
```返回[2]```.

**提示:** 如果众数超过1个，不需考虑输出顺序

**进阶:** 你可以不使用额外的空间吗？（假设由递归产生的隐式调用栈的开销不被计算在内）

## 题解 (Python)

### 1. 中序遍历
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def findMode(self, root: TreeNode) -> List[int]:
        nodes = []
        curr = root
        prev = 0
        cnt = 0
        max_cnt = 1
        modes = []

        while nodes or curr:
            while curr:
                nodes.append(curr)
                curr = curr.left

            curr = nodes.pop()

            if curr.val == prev:
                cnt += 1
            else:
                if cnt == max_cnt:
                    modes.append(prev)
                elif cnt > max_cnt:
                    modes = [prev]
                    max_cnt = cnt

                prev = curr.val
                cnt = 1

            curr = curr.right

        if cnt == max_cnt:
            modes.append(prev)
        elif cnt > max_cnt:
            modes = [prev]

        return modes
```
