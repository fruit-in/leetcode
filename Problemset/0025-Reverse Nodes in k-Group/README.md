# 25. Reverse Nodes in k-Group
Given the `head` of a linked list, reverse the nodes of the list `k` at a time, and return *the modified list*.

`k` is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is not a multiple of `k` then left-out nodes, in the end, should remain as it is.

You may not alter the values in the list's nodes, only nodes themselves may be changed.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/10/03/reverse_ex1.jpg)
<pre>
<strong>Input:</strong> head = [1,2,3,4,5], k = 2
<strong>Output:</strong> [2,1,4,3,5]
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2020/10/03/reverse_ex2.jpg)
<pre>
<strong>Input:</strong> head = [1,2,3,4,5], k = 3
<strong>Output:</strong> [3,2,1,4,5]
</pre>

#### Constraints:
* The number of nodes in the list is `n`.
* `1 <= k <= n <= 5000`
* `0 <= Node.val <= 1000`

**Follow-up:** Can you solve the problem in `O(1)` extra memory space?

## Solutions (Python)

### 1. Solution
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseKGroup(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        length = 0
        curr = head
        while curr is not None:
            length += 1
            curr = curr.next

        dummy = ListNode(next=head)
        grouptail = dummy

        for _ in range(length // k):
            grouphead = grouptail.next
            prev = grouphead
            for _ in range(k):
                prev = prev.next

            curr = grouphead
            for _ in range(k):
                temp = curr
                curr = curr.next
                temp.next = prev
                prev = temp

            grouptail.next = prev
            grouptail = grouphead

        return dummy.next
```
