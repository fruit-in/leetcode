# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def recoverFromPreorder(self, S: str) -> TreeNode:
        vals = [int(n) for n in S.split('-') if n != '']
        depths = [0]
        depth = 0

        for ch in S:
            if ch == '-':
                depth += 1
            elif depth != 0:
                depths.append(depth)
                depth = 0

        stack = []

        while vals:
            node = TreeNode(vals.pop(0))
            depth = depths.pop(0)

            while stack and stack[-1][1] >= depth:
                stack.pop()

            if stack and not stack[-1][0].left:
                stack[-1][0].left = node
            elif stack and not stack[-1][0].right:
                stack[-1][0].right = node

            stack.append((node, depth))

        return stack[0][0]
