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
