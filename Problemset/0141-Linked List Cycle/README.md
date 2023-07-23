# 141. Linked List Cycle
Given a linked list, determine if it has a cycle in it.

To represent a cycle in the given linked list, we use an integer ```pos``` which represents the position (0-indexed) in the linked list where tail connects to. If ```pos``` is ```-1```, then there is no cycle in the linked list.

#### Example 1:
<pre>
<strong>Input:</strong> head = [3,2,0,-4], pos = 1
<strong>Output:</strong> true
<strong>Explanation:</strong> There is a cycle in the linked list, where tail connects to the second node.
</pre>
![](https://assets.leetcode.com/uploads/2018/12/07/circularlinkedlist.png)

#### Example 2:
<pre>
<strong>Input:</strong> head = [1,2], pos = 0
<strong>Output:</strong> true
<strong>Explanation:</strong> There is a cycle in the linked list, where tail connects to the first node.
</pre>
![](https://assets.leetcode.com/uploads/2018/12/07/circularlinkedlist_test2.png)

#### Example 3:
<pre>
<strong>Input:</strong> head = [1], pos = -1
<strong>Output:</strong> false
<strong>Explanation:</strong> There is no cycle in the linked list.
</pre>
![](https://assets.leetcode.com/uploads/2018/12/07/circularlinkedlist_test3.png)

#### Follow up:
Can you solve it using *O(1)* (i.e. constant) memory?

## Solutions (Python)

### 1. Set
```Python3
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def hasCycle(self, head: ListNode) -> bool:
        nodes = set()
        while head:
            if head in nodes:
                return True
            nodes.add(head)
            head = head.next
        return False
```

### 2. Two Pointers
```Python3
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def hasCycle(self, head: ListNode) -> bool:
        slow, fast = head, head
        while fast and fast.next:
            slow = slow.next
            fast = fast.next.next
            if slow == fast:
                return True
        return False
```
