# 2487. 从链表中移除节点
给你一个链表的头节点 `head` 。

移除每个右侧有一个更大数值的节点。

返回修改后链表的头节点 `head` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2022/10/02/drawio.png)
<pre>
<strong>输入:</strong> head = [5,2,13,3,8]
<strong>输出:</strong> [13,8]
<strong>解释:</strong> 需要移除的节点是 5 ，2 和 3 。
- 节点 13 在节点 5 右侧。
- 节点 13 在节点 2 右侧。
- 节点 8 在节点 3 右侧。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> head = [1,1,1,1]
<strong>输出:</strong> [1,1,1,1]
<strong>解释:</strong> 每个节点的值都是 1 ，所以没有需要移除的节点。
</pre>

#### Constraints:
* 给定列表中的节点数目在范围 <code>[1, 10<sup>5</sup>]</code> 内
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
    def removeNodes(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is not None:
            head.next = self.removeNodes(head.next)
            if head.next is not None and head.val < head.next.val:
                head = head.next

        return head
```
