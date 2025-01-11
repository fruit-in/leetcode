# 1171. Remove Zero Sum Consecutive Nodes from Linked List
Given the `head` of a linked list, we repeatedly delete consecutive sequences of nodes that sum to `0` until there are no such sequences.

After doing so, return the head of the final linked list.  You may return any such answer.

(Note that in the examples below, all sequences are serializations of `ListNode` objects.)

#### Example 1:
<pre>
<strong>Input:</strong> head = [1,2,-3,3,1]
<strong>Output:</strong> [3,1]
<strong>Explanation:</strong> The answer [1,2,1] would also be accepted.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> head = [1,2,3,-3,4]
<strong>Output:</strong> [1,2,4]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> head = [1,2,3,-3,-2]
<strong>Output:</strong> [1]
</pre>

#### Constraints:
* The given linked list will contain between `1` and `1000` nodes.
* Each node in the linked list has `-1000 <= node.val <= 1000`.

## Solutions (Python)

### 1. Solution
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def removeZeroSumSublists(self, head: Optional[ListNode]) -> Optional[ListNode]:
        arr = []
        curr = head
        prefixsum = [0]
        prefixsumset = {0}

        while curr is not None:
            arr.append(curr.val)
            curr = curr.next
            prefixsum.append(prefixsum[-1] + arr[-1])
            if prefixsum[-1] in prefixsumset:
                arr.pop()
                tmp = prefixsum.pop()
                while tmp != prefixsum[-1]:
                    arr.pop()
                    prefixsumset.remove(prefixsum.pop())
            else:
                prefixsumset.add(prefixsum[-1])

        hair = ListNode()
        curr = hair

        for val in arr:
            curr.next = ListNode(val)
            curr = curr.next

        return hair.next
```
