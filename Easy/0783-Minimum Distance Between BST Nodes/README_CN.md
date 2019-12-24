# 783. 二叉搜索树结点最小距离
给定一个二叉搜索树的根结点 ```root```, 返回树中任意两节点的差的最小值。

#### 示例:
<pre>
<strong>输入:</strong> root = [4,2,6,1,3,null,null]
<strong>输出:</strong> 1
<strong>解释:</strong>
注意，root是树结点对象(TreeNode object)，而不是数组。

给定的树 [4,2,6,1,3,null,null] 可表示为下图:

          4
        /   \
      2      6
     / \
    1   3

最小的差值是 1, 它是节点1和节点2的差值, 也是节点3和节点2的差值。
</pre>

#### 注意:
1. 二叉树的大小范围在 ```2``` 到 ```100```。
2. 二叉树总是有效的，每个节点的值都是整数，且不重复。

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
    def minDiffInBST(self, root: TreeNode) -> int:
        nodes = []
        curr = root
        prev = float("-inf")
        min_diff = float("+inf")

        while nodes or curr:
            while curr:
                nodes.append(curr)
                curr = curr.left

            curr = nodes.pop()

            min_diff = min(min_diff, curr.val - prev)
            prev = curr.val

            curr = curr.right

        return min_diff
```
