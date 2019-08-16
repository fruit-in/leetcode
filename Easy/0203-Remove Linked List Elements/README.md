# 203. Remove Linked List Elements
Remove all elements from a linked list of integers that have value ***val***.

#### Example:
<pre>
<strong>Input:</strong> 1->2->6->3->4->5->6, <strong><em>val</em></strong> = 6
<strong>Output:</strong> 1->2->3->4->5
</pre>

## Solutions (Python)

### 1. Two Pointers
```Python3
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def removeElements(self, head: ListNode, val: int) -> ListNode:
        pre, cur = None, head
        while cur:
            if cur.val == val:
                if pre:
                    pre.next = cur.next
                else:
                    head = cur.next
            else:
                pre = cur
            cur = cur.next
        return head
```

### 2. Add a New Head
```Python3
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def removeElements(self, head: ListNode, val: int) -> ListNode:
        cur = ListNode(0)
        cur.next = head
        head = cur
        while cur and cur.next:
            if cur.next.val == val:
                cur.next = cur.next.next
            else:
                cur = cur.next
        return head.next
```

### 3. Recursion
```Python3
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def removeElements(self, head: ListNode, val: int) -> ListNode:
        if not head:
            return None
        elif head.val == val:
            return self.removeElements(head.next, val)
        else:
            head.next = self.removeElements(head.next, val)
            return head
```
