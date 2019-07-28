# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def constructMaximumBinaryTree(self, nums: List[int]) -> TreeNode:
        if nums:
            maxnum = max(nums)
            root = TreeNode(maxnum)
            index = nums.index(maxnum)
            root.left = self.constructMaximumBinaryTree(nums[:index])
            root.right = self.constructMaximumBinaryTree(nums[index + 1:])
            return root
        else:
            return None
