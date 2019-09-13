# 234. Palindrome Linked List
Given a singly linked list, determine if it is a palindrome.

#### Example 1:
<pre>
<strong>Input:</strong> 1->2
<strong>Output:</strong> false
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 1->2->2->1
<strong>Output:</strong> true
</pre>

#### Follow up:
Could you do it in O(n) time and O(1) space?

## Solutions (Python)

### 1. Convert to Array
```Python3
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def isPalindrome(self, head: ListNode) -> bool:
        vals = []
        curr = head
        while curr:
            vals.append(curr.val)
            curr = curr.next

        while head:
            if head.val != vals.pop():
                return False
            head = head.next
        return True
```

### 2. Clone & Reverse
```Python3
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def isPalindrome(self, head: ListNode) -> bool:
        if not head:
            return True

        head_ = ListNode(head.val)
        curr = head
        while curr.next:
            curr = curr.next
            head_new = ListNode(curr.val)
            head_new.next = head_
            head_ = head_new

        while head:
            if head.val != head_.val:
                return False
            head = head.next
            head_ = head_.next
        return True
```

### 3. Reverse Half
```Python3
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def isPalindrome(self, head: ListNode) -> bool:
        if not head:
            return True
        mid, end = head, head.next
        while end and end.next:
            mid = mid.next
            end = end.next.next

        prev = mid
        curr = mid.next
        while curr:
            next = curr.next
            curr.next = prev
            prev = curr
            curr = next

        while head != mid.next:
            if head.val != prev.val:
                return False
            head = head.next
            prev = prev.next
        return True
```
