# 2095. 删除链表的中间节点
给你一个链表的头节点 `head` 。**删除** 链表的 **中间节点** ，并返回修改后的链表的头节点 `head` 。

长度为 `n` 链表的中间节点是从头数起第 `⌊n / 2⌋` 个节点（下标从 **0** 开始），其中 `⌊x⌋` 表示小于或等于 `x` 的最大整数。

* 对于 `n` = `1`、`2`、`3`、`4` 和 `5` 的情况，中间节点的下标分别是 `0`、`1`、`1`、`2` 和 `2` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/11/16/eg1drawio.png)
<pre>
<strong>输入:</strong> head = [1,3,4,7,1,2,6]
<strong>输出:</strong> [1,3,4,1,2,6]
<strong>解释:</strong>
上图表示给出的链表。节点的下标分别标注在每个节点的下方。
由于 n = 7 ，值为 7 的节点 3 是中间节点，用红色标注。
返回结果为移除节点后的新链表。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/11/16/eg2drawio.png)
<pre>
<strong>输入:</strong> head = [1,2,3,4]
<strong>输出:</strong> [1,2,4]
<strong>解释:</strong>
上图表示给出的链表。
对于 n = 4 ，值为 3 的节点 2 是中间节点，用红色标注。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/11/16/eg3drawio.png)
<pre>
<strong>输入:</strong> head = [2,1]
<strong>输出:</strong> [2]
<strong>解释:</strong>
上图表示给出的链表。
对于 n = 2 ，值为 1 的节点 1 是中间节点，用红色标注。
值为 2 的节点 0 是移除节点 1 后剩下的唯一一个节点。
</pre>

#### 提示:
* 链表中节点的数目在范围 <code>[1, 10<sup>5</sup>]</code> 内
* <code>1 <= Node.val <= 10<sup>5</sup></code>

## 题解 (Python)

### 1. 题解
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
