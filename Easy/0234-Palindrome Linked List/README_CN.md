# 234. 回文链表
请判断一个链表是否为回文链表。

#### 示例 1:
<pre>
<strong>输入:</strong> 1->2
<strong>输出:</strong> false
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 1->2->2->1
<strong>输出:</strong> true
</pre>

#### 进阶:
你能否用 O(n) 时间复杂度和 O(1) 空间复杂度解决此题？

## 题解 (Python)

### 1. 转换为数组
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

### 2. 反转
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

### 3. 反转一半
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
