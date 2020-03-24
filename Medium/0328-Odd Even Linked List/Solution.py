# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def oddEvenList(self, head: ListNode) -> ListNode:
        if not head:
            return None

        even_head = head.next
        curr = head
        is_odd = True

        while curr.next:
            tmp = curr.next
            curr.next = tmp.next
            prev = curr
            curr = tmp
            is_odd = not is_odd

        if is_odd:
            curr.next = even_head
        else:
            prev.next = even_head

        return head
