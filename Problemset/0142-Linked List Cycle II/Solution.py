# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def detectCycle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None or head.next is None:
            return None

        slow = head.next
        fast = head.next.next

        while slow != fast:
            if fast is None or fast.next is None:
                return None

            slow = slow.next
            fast = fast.next.next

        while head != slow:
            head = head.next
            slow = slow.next

        return head
