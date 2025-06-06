# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def minimumOperations(self, root: Optional[TreeNode]) -> int:
        nodes = [root]
        ret = 0

        while nodes:
            nextlevel = []
            vals = []
            heap = []

            for node in nodes:
                if node.left:
                    nextlevel.append(node.left)
                    vals.append(node.left.val)
                    heapq.heappush(heap, (vals[-1], len(vals) - 1))
                if node.right:
                    nextlevel.append(node.right)
                    vals.append(node.right.val)
                    heapq.heappush(heap, (vals[-1], len(vals) - 1))

            for i in range(len(vals)):
                while heap[0][0] != vals[heap[0][1]]:
                    heapq.heappop(heap)

                j = heapq.heappop(heap)[1]
                if i != j:
                    heapq.heappush(heap, (vals[i], j))
                    vals[i], vals[j] = vals[j], vals[i]
                    ret += 1

            nodes = nextlevel

        return ret
