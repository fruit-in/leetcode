# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def flipMatchVoyage(self, root: Optional[TreeNode], voyage: List[int]) -> List[int]:
        stack = [root]
        i = 0
        ret = []

        while stack:
            if stack[-1].val != voyage[i]:
                return [-1]

            node = stack.pop()
            i += 1
            if i < len(voyage) and node.left and node.left.val != voyage[i]:
                node.left, node.right = node.right, node.left
                ret.append(node.val)
            if node.right:
                stack.append(node.right)
            if node.left:
                stack.append(node.left)

        return ret
