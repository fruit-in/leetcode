# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def sortList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        def forward(head: Optional[ListNode], step: int) -> Optional[ListNode]:
            i = 0

            while i < step and head:
                head = head.next
                i += 1

            return head

        def merge(head1: Optional[ListNode], len1: int, head2: Optional[ListNode], len2: int) -> (Optional[ListNode], Optional[ListNode]):
            i, j = 0, 0
            tail = hair = ListNode()

            while i < len1 or j < len2:
                if j >= len2 or (i < len1 and head1.val <= head2.val):
                    tail.next = head1
                    head1 = head1.next
                    i += 1
                else:
                    tail.next = head2
                    head2 = head2.next
                    j += 1
                tail = tail.next

            tail.next = None

            return (hair.next, tail)

        hair = ListNode(0, head)
        length = 0
        size = 1

        while head:
            length += 1
            head = head.next

        while size < length:
            newhead1 = hair.next
            tail = hair

            for i in range(0, length, size * 2):
                head1 = newhead1
                head2 = forward(head1, size)
                newhead1 = forward(head2, size)
                len1 = min(length - i, size)
                len2 = min(max(length - i - size, 0), size)
                tail.next, tail = merge(head1, len1, head2, len2)

            size *= 2

        return hair.next
