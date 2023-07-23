# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Solution:
    def findFrequentTreeSum(self, root: TreeNode) -> List[int]:
        def foo(root: TreeNode) -> int:
            sum = root.val
            sum += foo(root.left) if root.left else 0
            sum += foo(root.right) if root.right else 0

            freq[sum] = freq[sum] + 1 if sum in freq else 0

            return sum

        if not root:
            return []

        freq = {}
        foo(root)
        max_freq = max(freq.values())

        return [k for k, v in freq.items() if v == max_freq]
