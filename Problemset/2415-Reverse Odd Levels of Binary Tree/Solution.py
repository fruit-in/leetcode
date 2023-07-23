# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def reverseOddLevels(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        isOdd = False
        currLevel = [root]

        while currLevel:
            nextLevel = []
            if currLevel[0].left is not None:
                for node in currLevel:
                    nextLevel.append(node.left)
                    nextLevel.append(node.right)

            isOdd = not isOdd
            currLevel = nextLevel

            if isOdd:
                currVals = [node.val for node in currLevel[::-1]]
                for i in range(len(currLevel)):
                    currLevel[i].val = currVals[i]

        return root
