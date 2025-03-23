# 2487. Remove Nodes From Linked List
You are given the `head` of a linked list.

Remove every node which has a node with a greater value anywhere to the right side of it.

Return *the* `head` *of the modified linked list*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2022/10/02/drawio.png)
<pre>
<strong>Input:</strong> head = [5,2,13,3,8]
<strong>Output:</strong> [13,8]
<strong>Explanation:</strong> The nodes that should be removed are 5, 2 and 3.
- Node 13 is to the right of node 5.
- Node 13 is to the right of node 2.
- Node 8 is to the right of node 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> head = [1,1,1,1]
<strong>Output:</strong> [1,1,1,1]
<strong>Explanation:</strong> Every node has value 1, so no nodes are removed.
</pre>

#### Constraints:
* The number of the nodes in the given list is in the range <code>[1, 10<sup>5</sup>]</code>.
* <code>1 <= Node.val <= 10<sup>5</sup></code>

## Solutions (Python)

### 1. Solution
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def removeNodes(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is not None:
            head.next = self.removeNodes(head.next)
            if head.next is not None and head.val < head.next.val:
                head = head.next

        return head
```
