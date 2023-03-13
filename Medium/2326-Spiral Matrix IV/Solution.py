# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def spiralMatrix(self, m: int, n: int, head: Optional[ListNode]) -> List[List[int]]:
        curr = head
        i, j = 0, -1
        ret = [[-1] * n for _ in range(m)]

        while curr is not None:
            for (a, b) in [(0, 1)] * n + [(1, 0)] * (m - 1) + [(0, -1)] * (n - 1) + [(-1, 0)] * (m - 2):
                if curr is None:
                    break

                i += a
                j += b
                ret[i][j] = curr.val
                curr = curr.next

            m -= 2
            n -= 2

        return ret
