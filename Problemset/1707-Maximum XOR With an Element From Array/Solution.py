class TreeNode:
    def __init__(self, maxdepth=0, left=None, right=None, val=None):
        self.maxdepth = maxdepth
        self.left = left
        self.right = right
        self.val = val

    def insert(self, x, depth=0):
        if depth == self.maxdepth:
            self.val = x
            return

        bit = (x >> (self.maxdepth - depth - 1)) & 1

        if bit == 0:
            if not self.left:
                self.left = TreeNode(self.maxdepth)
            self.left.insert(x, depth + 1)
        else:
            if not self.right:
                self.right = TreeNode(self.maxdepth)
            self.right.insert(x, depth + 1)

    def getMaxXOR(self, x, depth=0):
        if depth == self.maxdepth:
            return self.val ^ x

        bit = (x >> (self.maxdepth - depth - 1)) & 1

        if (bit == 0 and not self.right) or (bit == 1 and self.left):
            return self.left.getMaxXOR(x, depth + 1)
        else:
            return self.right.getMaxXOR(x, depth + 1)


class Solution:
    def maximizeXor(self, nums: List[int], queries: List[List[int]]) -> List[int]:
        indices = sorted(range(len(queries)), key=lambda i: queries[i][1])
        root = TreeNode(ceil(log2(max(max(nums), max(x for x, _ in queries)))))
        answer = [-1] * len(queries)
        nums.sort(reverse=True)

        for i in indices:
            while nums and nums[-1] <= queries[i][1]:
                root.insert(nums.pop())

            if root.left or root.right:
                answer[i] = root.getMaxXOR(queries[i][0])

        return answer
