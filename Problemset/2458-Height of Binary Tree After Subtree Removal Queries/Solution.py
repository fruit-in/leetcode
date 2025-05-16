# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def treeQueries(self, root: Optional[TreeNode], queries: List[int]) -> List[int]:
        def dfs(root: TreeNode) -> int:
            nodes[root.val] = root
            root.height = 0

            if root.level >= len(leveltop2):
                leveltop2.append([])

            if root.left:
                root.left.level = root.level + 1
                root.height = dfs(root.left) + 1
            if root.right:
                root.right.level = root.level + 1
                root.height = max(root.height, dfs(root.right) + 1)

            leveltop2[root.level].append(root.height)
            leveltop2[root.level] = sorted(leveltop2[root.level])[-2:]

            return root.height

        nodes = {}
        leveltop2 = []
        root.level = 0
        answer = [0] * len(queries)

        dfs(root)

        for i in range(len(queries)):
            node = nodes[queries[i]]
            if len(leveltop2[node.level]) < 2:
                answer[i] = node.level - 1
            elif leveltop2[node.level][1] != node.height:
                answer[i] = leveltop2[node.level][1] + node.level
            else:
                answer[i] = leveltop2[node.level][0] + node.level

        return answer
