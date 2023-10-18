# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def countNodes(self, root: Optional[TreeNode]) -> int:
        low = 0
        high = 1
        curr = root
        root = TreeNode(left=TreeNode(), right=root)

        while curr is not None:
            low = (low << 1) + 1
            high = (high << 1) + 1
            curr = curr.right

        while low < high:
            mid = (low + high) // 2
            curr = root
            flag = True

            for bit in bin(mid)[2:]:
                if bit == '0':
                    curr = curr.left
                else:
                    curr = curr.right

                if curr is None:
                    flag = False
                    break

            if flag:
                low = mid + 1
            else:
                high = mid

        return low - 1
