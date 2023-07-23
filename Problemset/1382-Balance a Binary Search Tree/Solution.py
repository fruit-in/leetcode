# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def balanceBST(self, root: TreeNode) -> TreeNode:
        def foo(vals: List[int]) -> TreeNode:
            if not vals:
                return None

            mid = len(vals) // 2

            return TreeNode(vals[mid], foo(vals[:mid]), foo(vals[mid + 1:]))

        curr = root
        stack = []
        vals = []

        while stack or curr:
            while curr:
                stack.append(curr)
                curr = curr.left

            curr = stack.pop()
            vals.append(curr.val)

            curr = curr.right

        return foo(vals)
