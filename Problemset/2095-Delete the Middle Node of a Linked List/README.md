# 2095. Delete the Middle Node of a Linked List
You are given the `head` of a linked list. **Delete** the **middle node**, and return *the* `head` *of the modified linked list*.

The **middle node** of a linked list of size `n` is the <code>⌊n / 2⌋<sup>th</sup></code> node from the **start** using **0-based indexing**, where `⌊x⌋` denotes the largest integer less than or equal to `x`.

* For `n` = `1`, `2`, `3`, `4`, and `5`, the middle nodes are `0`, `1`, `1`, `2`, and `2`, respectively.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/11/16/eg1drawio.png)
<pre>
<strong>Input:</strong> head = [1,3,4,7,1,2,6]
<strong>Output:</strong> [1,3,4,1,2,6]
<strong>Explanation:</strong>
The above figure represents the given linked list. The indices of the nodes are written below.
Since n = 7, node 3 with value 7 is the middle node, which is marked in red.
We return the new list after removing this node.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/11/16/eg2drawio.png)
<pre>
<strong>Input:</strong> head = [1,2,3,4]
<strong>Output:</strong> [1,2,4]
<strong>Explanation:</strong>
The above figure represents the given linked list.
For n = 4, node 2 with value 3 is the middle node, which is marked in red.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/11/16/eg3drawio.png)
<pre>
<strong>Input:</strong> head = [2,1]
<strong>Output:</strong> [2]
<strong>Explanation:</strong>
The above figure represents the given linked list.
For n = 2, node 1 with value 1 is the middle node, which is marked in red.
Node 0 with value 2 is the only node remaining after removing node 1.
</pre>

#### Constraints:
* The number of nodes in the list is in the range <code>[1, 10<sup>5</sup>]</code>.
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
    def deleteMiddle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head.next is None:
            return None

        n = 0
        curr = head

        while curr is not None:
            n += 1
            curr = curr.next

        x = n // 2
        curr = ListNode(next=head)

        for _ in range(x):
            curr = curr.next

        curr.next = curr.next.next

        return head
```
