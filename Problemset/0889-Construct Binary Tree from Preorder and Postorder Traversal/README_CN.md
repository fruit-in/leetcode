# 889. 根据前序和后序遍历构造二叉树
返回与给定的前序和后序遍历匹配的任何二叉树。

`pre` 和 `post` 遍历中的值是不同的正整数。

#### 示例:
<pre>
<strong>输入:</strong> pre = [1,2,4,5,3,6,7], post = [4,5,2,6,7,3,1]
<strong>输出:</strong> [1,2,3,4,5,6,7]
</pre>

#### 提示:
* `1 <= pre.length == post.length <= 30`
* `pre[]` 和 `post[]` 都是 `1, 2, ..., pre.length` 的排列
* 每个输入保证至少有一个答案。如果有多个答案，可以返回其中一个。

## 题解 (Python)

### 1. 递归
```Python
# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def constructFromPrePost(self, pre: List[int], post: List[int]) -> TreeNode:
        if not pre:
            return None
        elif len(pre) == 1:
            return TreeNode(pre[0])

        i = post.index(pre[1])

        return TreeNode(
            pre[0],
            self.constructFromPrePost(pre[1:i + 2], post[:i + 1]),
            self.constructFromPrePost(pre[i + 2:], post[i + 1:-1])
        )
```
