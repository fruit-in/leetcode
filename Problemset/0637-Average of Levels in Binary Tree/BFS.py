# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def averageOfLevels(self, root: TreeNode) -> List[float]:
        averages = []
        curr_level = [root]

        while curr_level:
            averages.append(sum(node.val for node in curr_level) / len(curr_level))

            temp = [node.left for node in curr_level if node.left]
            temp.extend(node.right for node in curr_level if node.right)
            curr_level = temp

        return averages
