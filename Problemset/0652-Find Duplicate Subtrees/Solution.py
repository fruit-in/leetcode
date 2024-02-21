# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def findDuplicateSubtrees(self, root: Optional[TreeNode]) -> List[Optional[TreeNode]]:
        count = {}
        ret = []

        def dfs(root: Optional[TreeNode]) -> str:
            if root is None:
                return "()"

            s = "{}({})({})".format(root.val, dfs(root.left), dfs(root.right))
            count[s] = count.get(s, 0) + 1

            if count[s] == 2:
                ret.append(root)

            return s

        dfs(root)

        return ret
