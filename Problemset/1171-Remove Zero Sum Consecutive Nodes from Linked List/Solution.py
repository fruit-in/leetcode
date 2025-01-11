# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def removeZeroSumSublists(self, head: Optional[ListNode]) -> Optional[ListNode]:
        arr = []
        curr = head
        prefixsum = [0]
        prefixsumset = {0}

        while curr is not None:
            arr.append(curr.val)
            curr = curr.next
            prefixsum.append(prefixsum[-1] + arr[-1])
            if prefixsum[-1] in prefixsumset:
                arr.pop()
                tmp = prefixsum.pop()
                while tmp != prefixsum[-1]:
                    arr.pop()
                    prefixsumset.remove(prefixsum.pop())
            else:
                prefixsumset.add(prefixsum[-1])

        hair = ListNode()
        curr = hair

        for val in arr:
            curr.next = ListNode(val)
            curr = curr.next

        return hair.next
