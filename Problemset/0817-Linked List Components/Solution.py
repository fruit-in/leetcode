# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def numComponents(self, head: ListNode, G: List[int]) -> int:
        G = set(G)
        curr = head
        ret = 0

        while curr:
            if curr.val in G and (not curr.next or curr.next.val not in G):
                ret += 1

            curr = curr.next

        return ret
