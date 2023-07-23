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
