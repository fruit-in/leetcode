# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseEvenLengthGroups(self, head: Optional[ListNode]) -> Optional[ListNode]:
        curr = head
        vals = []

        while curr is not None:
            vals.append(curr.val)
            curr = curr.next

        i = 0
        size = 1

        while i < len(vals):
            j = min(i + size, len(vals))
            if (j - i) % 2 == 0:
                vals[i:j] = vals[i:j][::-1]
            i += size
            size += 1

        curr = head

        for val in vals:
            curr.val = val
            curr = curr.next

        return head
