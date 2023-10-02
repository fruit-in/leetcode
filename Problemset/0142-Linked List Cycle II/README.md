# 142. Linked List Cycle II
Given the `head` of a linked list, return *the node where the cycle begins. If there is no cycle, return* `null`.

There is a cycle in a linked list if there is some node in the list that can be reached again by continuously following the `next` pointer. Internally, `pos` is used to denote the index of the node that tail's `next` pointer is connected to (**0-indexed**). It is `-1` if there is no cycle. **Note that** `pos` **is not passed as a parameter**.

**Do not modify** the linked list.

#### Example 1:
![](https://assets.leetcode.com/uploads/2018/12/07/circularlinkedlist.png)
<pre>
<strong>Input:</strong> head = [3,2,0,-4], pos = 1
<strong>Output:</strong> tail connects to node index 1
<strong>Explanation:</strong> There is a cycle in the linked list, where tail connects to the second node.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2018/12/07/circularlinkedlist_test2.png)
<pre>
<strong>Input:</strong> head = [1,2], pos = 0
<strong>Output:</strong> tail connects to node index 0
<strong>Explanation:</strong> There is a cycle in the linked list, where tail connects to the first node.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2018/12/07/circularlinkedlist_test3.png)
<pre>
<strong>Input:</strong> head = [1], pos = -1
<strong>Output:</strong> no cycle
<strong>Explanation:</strong> There is no cycle in the linked list.
</pre>

#### Constraints:
* The number of the nodes in the list is in the range <code>[0, 10<sup>4</sup>]</code>.
* <code>-10<sup>5</sup> <= Node.val <= 10<sup>5</sup></code>
* `pos` is `-1` or a **valid index** in the linked-list.

**Follow up:** Can you solve it using `O(1)` (i.e. constant) memory?

## Solutions (Python)

### 1. Solution
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def detectCycle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None or head.next is None:
            return None

        slow = head.next
        fast = head.next.next

        while slow != fast:
            if fast is None or fast.next is None:
                return None

            slow = slow.next
            fast = fast.next.next

        while head != slow:
            head = head.next
            slow = slow.next

        return head
```
