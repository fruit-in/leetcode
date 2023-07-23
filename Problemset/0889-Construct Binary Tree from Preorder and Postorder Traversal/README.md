# 889. Construct Binary Tree from Preorder and Postorder Traversal
Return any binary tree that matches the given preorder and postorder traversals.

Values in the traversals `pre` and `post` are distinct positive integers.

#### Example 1:
<pre>
<strong>Input:</strong> pre = [1,2,4,5,3,6,7], post = [4,5,2,6,7,3,1]
<strong>Output:</strong> [1,2,3,4,5,6,7]
</pre>

#### Note:
* `1 <= pre.length == post.length <= 30`
* `pre[]` and `post[]` are both permutations of `1, 2, ..., pre.length`.
* It is guaranteed an answer exists. If there exists multiple answers, you can return any of them.

## Solutions (Python)

### 1. Recursion
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
