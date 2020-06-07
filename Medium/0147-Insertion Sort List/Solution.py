# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def insertionSortList(self, head: ListNode) -> ListNode:
        if not head:
            return head

        dummy = ListNode(next=head)
        tail = head
        while tail.next and tail.val <= tail.next.val:
            tail = tail.next

        while tail.next:
            curr = tail.next
            tail.next = curr.next
            while tail.next and tail.val <= tail.next.val:
                tail = tail.next

            node = dummy

            while curr.val >= node.next.val:
                node = node.next

            curr.next = node.next
            node.next = curr

        return dummy.next
