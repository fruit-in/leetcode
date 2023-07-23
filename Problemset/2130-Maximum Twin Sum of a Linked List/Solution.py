# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def pairSum(self, head: Optional[ListNode]) -> int:
        curr = head
        vals = []

        while curr:
            vals.append(curr.val)
            curr = curr.next

        return max(vals[i] + vals[-1 - i] for i in range(len(vals) // 2))
