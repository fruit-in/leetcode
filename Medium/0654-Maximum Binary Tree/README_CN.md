# 654. 最大二叉树
给定一个不含重复元素的整数数组。一个以此数组构建的最大二叉树定义如下：
1. 二叉树的根是数组中的最大元素。
2. 左子树是通过数组中最大值左边部分构造出的最大二叉树。
3. 右子树是通过数组中最大值右边部分构造出的最大二叉树。

通过给定的数组构建最大二叉树，并且输出这个树的根节点。

#### 示例:
<pre>
<strong>输入:</strong> [3,2,1,6,0,5]
<strong>输出:</strong> 返回下面这棵树的根节点:

      6
    /   \
   3     5
    \    /
     2  0
       \
        1
</pre>

#### 提示:
1. 给定的数组的大小在 [1, 1000] 之间。

## 题解 (Python)

### 1. 递归
```Python3
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def constructMaximumBinaryTree(self, nums: List[int]) -> TreeNode:
        if nums:
            maxnum = max(nums)
            root = TreeNode(maxnum)
            index = nums.index(maxnum)
            root.left = self.constructMaximumBinaryTree(nums[:index])
            root.right = self.constructMaximumBinaryTree(nums[index + 1:])
            return root
        else:
            return None
```
