# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def closestNodes(self, root: Optional[TreeNode], queries: List[int]) -> List[List[int]]:
        vals = []
        answer = []

        def dfs(root):
            if root is not None:
                dfs(root.left)
                vals.append(root.val)
                dfs(root.right)

        dfs(root)

        for x in queries:
            i = bisect.bisect(vals, x)
            j = bisect.bisect_left(vals, x)
            answer.append([vals[i - 1] if i > 0 else -1,
                          vals[j] if j < len(vals) else -1])

        return answer
