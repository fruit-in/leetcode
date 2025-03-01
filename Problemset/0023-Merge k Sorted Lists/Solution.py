# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

import heapq


class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        heap = [(lists[i].val, i)
                for i in range(len(lists)) if lists[i] is not None]
        heapq.heapify(heap)
        hair = ListNode()
        curr = hair

        while len(heap) > 0:
            _, i = heapq.heappop(heap)
            curr.next = lists[i]
            curr = curr.next
            lists[i] = lists[i].next
            if lists[i] is not None:
                heapq.heappush(heap, (lists[i].val, i))

        return hair.next
