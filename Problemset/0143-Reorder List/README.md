# 143. Reorder List
You are given the head of a singly linked-list. The list can be represented as:

L<sub>0</sub> → L<sub>1</sub> → … → L<sub>n - 1</sub> → L<sub>n</sub>

*Reorder the list to be on the following form:*

L<sub>0</sub> → L<sub>n</sub> → L<sub>1</sub> → L<sub>n - 1</sub> → L<sub>2</sub> → L<sub>n - 2</sub> → …

You may not modify the values in the list's nodes. Only nodes themselves may be changed.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/03/04/reorder1linked-list.jpg)
<pre>
<strong>Input:</strong> head = [1,2,3,4]
<strong>Output:</strong> [1,4,2,3]
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/03/09/reorder2-linked-list.jpg)
<pre>
<strong>Input:</strong> head = [1,2,3,4,5]
<strong>Output:</strong> [1,5,2,4,3]
</pre>

#### Constraints:
* The number of nodes in the list is in the range <code>[1, 5 * 10<sup>4</sup>]</code>.
* `1 <= Node.val <= 1000`

## Solutions (Python)

### 1. Solution
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        """
        Do not return anything, modify head in-place instead.
        """
        length = 0
        curr = head
        while curr is not None:
            length += 1
            curr = curr.next

        prev = None
        curr = head
        for _ in range((length + 1) // 2):
            prev = curr
            curr = curr.next

        prev.next = None
        prev = None
        for _ in range(length // 2):
            temp = curr
            curr = curr.next
            temp.next = prev
            prev = temp

        curr0 = head
        curr1 = prev
        for _ in range(length // 2):
            prev0 = curr0
            prev1 = curr1
            curr0 = curr0.next
            curr1 = curr1.next
            prev0.next = prev1
            prev1.next = curr0
```
