# 2074. 反转偶数长度组的节点
给你一个链表的头节点 `head` 。

链表中的节点 **按顺序** 划分成若干 **非空** 组，这些非空组的长度构成一个自然数序列（`1, 2, 3, 4, ...`）。一个组的 **长度** 就是组中分配到的节点数目。换句话说：

* 节点 `1` 分配给第一组
* 节点 `2` 和 `3` 分配给第二组
* 节点 `4`、`5` 和 `6` 分配给第三组，以此类推

注意，最后一组的长度可能小于或者等于 `1 + 倒数第二组的长度` 。

**反转** 每个 **偶数** 长度组中的节点，并返回修改后链表的头节点 `head` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/10/25/eg1.png)
<pre>
<strong>输入:</strong> head = [5,2,6,3,9,1,7,3,8,4]
<strong>输出:</strong> [5,6,2,3,9,1,4,8,3,7]
<strong>解释:</strong>
- 第一组长度为 1 ，奇数，没有发生反转。
- 第二组长度为 2 ，偶数，节点反转。
- 第三组长度为 3 ，奇数，没有发生反转。
- 最后一组长度为 4 ，偶数，节点反转。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/10/25/eg2.png)
<pre>
<strong>输入:</strong> head = [1,1,0,6]
<strong>输出:</strong> [1,0,1,6]
<strong>解释:</strong>
- 第一组长度为 1 ，没有发生反转。
- 第二组长度为 2 ，节点反转。
- 最后一组长度为 1 ，没有发生反转。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/10/28/eg3.png)
<pre>
<strong>输入:</strong> head = [2,1]
<strong>输出:</strong> [2,1]
<strong>解释:</strong>
- 第一组长度为 1 ，没有发生反转。
- 最后一组长度为 1 ，没有发生反转。
</pre>

#### 提示:
* 链表中节点数目范围是 <code>[1, 10<sup>5</sup>]</code>
* <code>0 <= Node.val <= 10<sup>5</sup></code>

## 题解 (Python)

### 1. 题解
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseEvenLengthGroups(self, head: Optional[ListNode]) -> Optional[ListNode]:
        curr = head
        vals = []

        while curr is not None:
            vals.append(curr.val)
            curr = curr.next

        i = 0
        size = 1

        while i < len(vals):
            j = min(i + size, len(vals))
            if (j - i) % 2 == 0:
                vals[i:j] = vals[i:j][::-1]
            i += size
            size += 1

        curr = head

        for val in vals:
            curr.val = val
            curr = curr.next

        return head
```
