# 61. Rotate List
Given a linked list, rotate the list to the right by *k* places, where *k* is non-negative.

#### Example 1:
<pre>
<strong>Input:</strong> 1->2->3->4->5->NULL, k = 2
<strong>Output:</strong> 4->5->1->2->3->NULL
<strong>Explanation:</strong>
rotate 1 steps to the right: 5->1->2->3->4->NULL
rotate 2 steps to the right: 4->5->1->2->3->NULL
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 0->1->2->NULL, k = 4
<strong>Output:</strong> 2->0->1->NULL
<strong>Explanation:</strong>
rotate 1 steps to the right: 2->0->1->NULL
rotate 2 steps to the right: 1->2->0->NULL
rotate 3 steps to the right: 0->1->2->NULL
rotate 4 steps to the right: 2->0->1->NULL
</pre>

## Solutions (Python)

### 1. Solution
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def rotateRight(self, head: ListNode, k: int) -> ListNode:
        if not head:
            return None

        length = 1
        curr = head
        while curr.next:
            curr = curr.next
            length += 1
        curr.next = head

        for _ in range(length - k % length - 1):
            head = head.next

        new_head = head.next
        head.next = None

        return new_head
```
