# 328. Odd Even Linked List
Given a singly linked list, group all odd nodes together followed by the even nodes. Please note here we are talking about the node number and not the value in the nodes.

You should try to do it in place. The program should run in O(1) space complexity and O(nodes) time complexity.

#### Example 1:
<pre>
<strong>Input:</strong> 1->2->3->4->5->NULL
<strong>Output:</strong> 1->3->5->2->4->NULL
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 2->1->3->5->6->4->7->NULL
<strong>Output:</strong> 2->3->6->7->1->5->4->NULL
</pre>

#### Note:
* The relative order inside both the even and odd groups should remain as it was in the input.
* The first node is considered odd, the second node even and so on ...

## Solutions (Python)

### 1. Solution
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def oddEvenList(self, head: ListNode) -> ListNode:
        if not head:
            return None

        even_head = head.next
        curr = head
        is_odd = True

        while curr.next:
            tmp = curr.next
            curr.next = tmp.next
            prev = curr
            curr = tmp
            is_odd = not is_odd

        if is_odd:
            curr.next = even_head
        else:
            prev.next = even_head

        return head
```
