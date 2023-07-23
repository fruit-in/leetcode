# 206. Reverse Linked List
Reverse a singly linked list.

#### Example:
<pre>
<strong>Input:</strong> 1->2->3->4->5->NULL
<strong>Output:</strong> 5->4->3->2->1->NULL
</pre>

#### Follow up:
A linked list can be reversed either iteratively or recursively. Could you implement both?

## Solutions (Python)

### 1. Recursion
```Python3
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def reverseList(self, head: ListNode) -> ListNode:
        if not head or not head.next:
            return head
        new_head = self.reverseList(head.next)
        head.next.next = head
        head.next = None
        return new_head
```

### 2. Iteration
```Python3
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def reverseList(self, head: ListNode) -> ListNode:
        curr = head
        prev = None
        while curr:
            next = curr.next
            curr.next = prev
            prev = curr
            curr = next
        return prev
```
