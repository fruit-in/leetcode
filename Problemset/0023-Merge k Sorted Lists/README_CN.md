# 23. 合并 K 个升序链表
给你一个链表数组，每个链表都已经按升序排列。

请你将所有链表合并到一个升序链表中，返回合并后的链表。

#### 示例 1:
<pre>
<strong>输入:</strong> lists = [[1,4,5],[1,3,4],[2,6]]
<strong>输出:</strong> [1,1,2,3,4,4,5,6]
<strong>解释:</strong> 链表数组如下：
[
  1->4->5,
  1->3->4,
  2->6
]
将它们合并到一个有序链表中得到。
1->1->2->3->4->4->5->6
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> list = []
<strong>输出:</strong> []
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> [[]]
<strong>输出:</strong> []
</pre>

#### 提示:
* `k == lists.length`
* <code>0 <= k <= 10<sup>4</sup></code>
* `0 <= lists[i].length <= 500`
* <code>-10<sup>4</sup> <= lists[i][j] <= 10<sup>4</sup></code>
* `lists[i]` 按 **升序** 排列
* `lists[i].length` 的总和不超过 <code>10<sup>4</sup></code>

## 题解 (Python)

### 1. 题解
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
