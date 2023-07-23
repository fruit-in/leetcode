# 725. Split Linked List in Parts
Given a (singly) linked list with head node ```root```, write a function to split the linked list into ```k``` consecutive linked list "parts".

The length of each part should be as equal as possible: no two parts should have a size differing by more than 1. This may lead to some parts being null.

The parts should be in order of occurrence in the input list, and parts occurring earlier should always have a size greater than or equal parts occurring later.

Return a List of ListNode's representing the linked list parts that are formed.

Examples 1->2->3->4, k = 5 // 5 equal parts [ [1], [2], [3], [4], null ]

#### Example 1:
<pre>
<strong>Input:</strong>
root = [1, 2, 3], k = 5
<strong>Output:</strong> [[1],[2],[3],[],[]]
<strong>Explanation:</strong>
The input and each element of the output are ListNodes, not arrays.
For example, the input root has root.val = 1, root.next.val = 2, \root.next.next.val = 3, and root.next.next.next = null.
The first element output[0] has output[0].val = 1, output[0].next = null.
The last element output[4] is null, but it's string representation as a ListNode is [].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
root = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], k = 3
<strong>Output:</strong> [[1, 2, 3, 4], [5, 6, 7], [8, 9, 10]]
<strong>Explanation:</strong>
The input has been split into consecutive parts with size difference at most 1, and earlier parts are a larger size than the later parts.
</pre>

#### Note:
* The length of ```root``` will be in the range ```[0, 1000]```.
* Each value of a node in the input will be an integer in the range ```[0, 999]```.
* ```k``` will be an integer in the range ```[1, 50]```.

## Solutions (Python)

### 1. Solution
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def splitListToParts(self, root: ListNode, k: int) -> List[ListNode]:
        curr = root
        length = 0
        while curr:
            length += 1
            curr = curr.next

        curr = root
        ret = []
        for i in range(k):
            ret.append(curr)
            for _ in range(length // k + (i < length % k) - 1):
                curr = curr.next
            if curr:
                curr.next, curr = None, curr.next

        return ret
```
