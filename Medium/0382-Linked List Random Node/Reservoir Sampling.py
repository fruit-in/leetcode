# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:

    def __init__(self, head: ListNode):
        """
        @param head The linked list's head.
        Note that the head is guaranteed to be not null, so it contains at least one node.
        """
        self.head = head


    def getRandom(self) -> int:
        """
        Returns a random node's value.
        """
        curr = self.head
        cnt = 0
        ret = 0

        while curr:
            cnt += 1
            if random.randint(1, cnt) == cnt:
                ret = curr.val
            curr = curr.next

        return ret


# Your Solution object will be instantiated and called as such:
# obj = Solution(head)
# param_1 = obj.getRandom()
