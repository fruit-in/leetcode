# 876. Middle of the Linked List
Given a non-empty, singly linked list with head node <code>head</code>, return a middle node of linked list.

If there are two middle nodes, return the second middle node.

#### Example 1:
<pre>
<strong>Input:</strong> [1,2,3,4,5]
<strong>Output:</strong> Node 3 from this list (Serialization: [3,4,5])
The returned node has value 3.  (The judge's serialization of this node is [3,4,5]).
Note that we returned a ListNode object ans, such that:
ans.val = 3, ans.next.val = 4, ans.next.next.val = 5, and ans.next.next.next = NULL.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [1,2,3,4,5,6]
<strong>Output:</strong> Node 4 from this list (Serialization: [4,5,6])
Since the list has two middle nodes with values 3 and 4, we return the second one.
</pre>

#### Note:
* The number of nodes in the given list will be between <code>1</code> and <code>100</code>.

## Solutions (Python)

### 1. Two Pointers
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def middleNode(self, head: ListNode) -> ListNode:
        p1 = head
        p2 = head
        while p2 and p2.next:
            p1 = p1.next
            p2 = p2.next.next
        return p1
```

### 2. Count Nodes
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def middleNode(self, head: ListNode) -> ListNode:
        list_len = 0
        p = head
        while p:
            list_len += 1
            p = p.next
        for i in range(list_len // 2):
            head = head.next
        return head
```
