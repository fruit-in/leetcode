# 23. Merge k Sorted Lists
You are given an array of `k` linked-lists `lists`, each linked-list is sorted in ascending order.

*Merge all the linked-lists into one sorted linked-list and return it*.

#### Example 1:
<pre>
<strong>Input:</strong> lists = [[1,4,5],[1,3,4],[2,6]]
<strong>Output:</strong> [1,1,2,3,4,4,5,6]
<strong>Explanation:</strong> The linked-lists are:
[
  1->4->5,
  1->3->4,
  2->6
]
merging them into one sorted list:
1->1->2->3->4->4->5->6
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> list = []
<strong>Output:</strong> []
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> [[]]
<strong>Output:</strong> []
</pre>

#### Constraints:
* `k == lists.length`
* <code>0 <= k <= 10<sup>4</sup></code>
* `0 <= lists[i].length <= 500`
* <code>-10<sup>4</sup> <= lists[i][j] <= 10<sup>4</sup></code>
* `lists[i]` is sorted in **ascending order**.
* The sum of `lists[i].length` will not exceed <code>10<sup>4</sup></code>.

## Solutions (Python)

### 1. Solution
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

import heapq


class Solution:
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        heap = [(lists[i].val, i)
                for i in range(len(lists)) if lists[i] is not None]
        heapq.heapify(heap)
        hair = ListNode()
        curr = hair

        while len(heap) > 0:
            _, i = heapq.heappop(heap)
            curr.next = lists[i]
            curr = curr.next
            lists[i] = lists[i].next
            if lists[i] is not None:
                heapq.heappush(heap, (lists[i].val, i))

        return hair.next
```
