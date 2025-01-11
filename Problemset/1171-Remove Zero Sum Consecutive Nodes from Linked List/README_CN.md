# 1171. 从链表中删去总和值为零的连续节点
给你一个链表的头节点 `head`，请你编写代码，反复删去链表中由 **总和** 值为 `0` 的连续节点组成的序列，直到不存在这样的序列为止。

删除完毕后，请你返回最终结果链表的头节点。

你可以返回任何满足题目要求的答案。

（注意，下面示例中的所有序列，都是对 `ListNode` 对象序列化的表示。）

#### 示例 1:
<pre>
<strong>输入:</strong> head = [1,2,-3,3,1]
<strong>输出:</strong> [3,1]
<strong>提示:</strong> 答案 [1,2,1] 也是正确的。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> head = [1,2,3,-3,4]
<strong>输出:</strong> [1,2,4]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> head = [1,2,3,-3,-2]
<strong>输出:</strong> [1]
</pre>

#### 提示:
* 给你的链表中可能有 `1` 到 `1000` 个节点。
* 对于链表中的每个节点，节点的值：`-1000 <= node.val <= 1000`.

## 题解 (Python)

### 1. 题解
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
