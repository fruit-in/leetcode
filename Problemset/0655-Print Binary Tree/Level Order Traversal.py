# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def printTree(self, root: TreeNode) -> List[List[str]]:
        curr_level = [root]
        vals = []

        while any(curr_level):
            curr_vals = []
            next_level = []

            for node in curr_level:
                if node:
                    curr_vals.append(str(node.val))
                    next_level.append(node.left)
                    next_level.append(node.right)
                else:
                    curr_vals.append("")
                    next_level.append(None)
                    next_level.append(None)

            vals.append(curr_vals)
            curr_level = next_level

        indices = [i for i in range(0, 2 ** len(vals), 2)]
        ret = [[""] * (2 ** len(vals) - 1) for _ in range(len(vals))]

        for i in range(-1, -1 - len(vals), -1):
            new_indices = []

            while indices:
                index0 = indices.pop()
                ret[i][index0] = vals[i].pop()
                if indices:
                    index1 = indices.pop()
                    ret[i][index1] = vals[i].pop()
                    new_indices.append((index0 + index1) // 2)

            indices = new_indices[::-1]

        return ret
