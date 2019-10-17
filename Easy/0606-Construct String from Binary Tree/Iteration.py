# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def tree2str(self, t: TreeNode) -> str:
        if not t:
            return ""

        ret = ""
        nodes = [t]
        visited = set()

        while nodes:
            cur = nodes.pop()
            if cur not in visited:
                visited.add(cur)
                nodes.append(cur)
                ret += "(" + str(cur.val)

                if cur.right:
                    nodes.append(cur.right)
                if cur.left:
                    nodes.append(cur.left)
                if not cur.left and cur.right:
                    ret += "()"
            else:
                ret += ")"

        return ret[1:-1]
