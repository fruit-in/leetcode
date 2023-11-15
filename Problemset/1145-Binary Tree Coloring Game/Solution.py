# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def btreeGameWinningMove(self, root: Optional[TreeNode], n: int, x: int) -> bool:
        nodes = [root]
        nodex = root

        while nodes != []:
            curr = nodes.pop()

            if curr.left is not None:
                if curr.left.val == x:
                    nodex = curr.left
                    break
                nodes.append(curr.left)
            if curr.right is not None:
                if curr.right.val == x:
                    nodex = curr.right
                    break
                nodes.append(curr.right)

        for node in [root, nodex.left, nodex.right]:
            if node is None or node.val == x:
                continue

            nodes = [node]
            count = 0

            while nodes != []:
                curr = nodes.pop()
                count += 1

                if curr.left is not None and curr.left.val != x:
                    nodes.append(curr.left)
                if curr.right is not None and curr.right.val != x:
                    nodes.append(curr.right)

            if count > n // 2:
                return True

        return False
