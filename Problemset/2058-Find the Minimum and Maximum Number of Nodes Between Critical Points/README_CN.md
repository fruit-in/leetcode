# 2058. 找出临界点之间的最小和最大距离
链表中的 **临界点** 定义为一个 **局部极大值点** **或** **局部极小值点** 。

如果当前节点的值 **严格大于** 前一个节点和后一个节点，那么这个节点就是一个  **局部极大值点** 。

如果当前节点的值 **严格小于** 前一个节点和后一个节点，那么这个节点就是一个  **局部极小值点** 。

注意：节点只有在同时存在前一个节点和后一个节点的情况下，才能成为一个 **局部极大值点 / 极小值点** 。

给你一个链表 `head` ，返回一个长度为 2 的数组 `[minDistance, maxDistance]` ，其中 `minDistance` 是任意两个不同临界点之间的最小距离，`maxDistance` 是任意两个不同临界点之间的最大距离。如果临界点少于两个，则返回 `[-1，-1]` 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/10/13/a1.png)
<pre>
<strong>输入:</strong> head = [3,1]
<strong>输出:</strong> [-1,-1]
<strong>解释:</strong> 链表 [3,1] 中不存在临界点。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/10/13/a2.png)
<pre>
<strong>输入:</strong> head = [5,3,1,2,5,1,2]
<strong>输出:</strong> [1,3]
<strong>解释:</strong> 存在三个临界点：
- [5,3,1,2,5,1,2]：第三个节点是一个局部极小值点，因为 1 比 3 和 2 小。
- [5,3,1,2,5,1,2]：第五个节点是一个局部极大值点，因为 5 比 2 和 1 大。
- [5,3,1,2,5,1,2]：第六个节点是一个局部极小值点，因为 1 比 5 和 2 小。
第五个节点和第六个节点之间距离最小。minDistance = 6 - 5 = 1 。
第三个节点和第六个节点之间距离最大。maxDistance = 6 - 3 = 3 。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/10/14/a5.png)
<pre>
<strong>输入:</strong> head = [1,3,2,2,3,2,2,2,7]
<strong>输出:</strong> [3,3]
<strong>解释:</strong> 存在两个临界点：
- [1,3,2,2,3,2,2,2,7]：第二个节点是一个局部极大值点，因为 3 比 1 和 2 大。
- [1,3,2,2,3,2,2,2,7]：第五个节点是一个局部极大值点，因为 3 比 2 和 2 大。
最小和最大距离都存在于第二个节点和第五个节点之间。
因此，minDistance 和 maxDistance 是 5 - 2 = 3 。
注意，最后一个节点不算一个局部极大值点，因为它之后就没有节点了。
</pre>

#### 示例 4:
![](https://assets.leetcode.com/uploads/2021/10/13/a4.png)
<pre>
<strong>输入:</strong> head = [2,3,3,2]
<strong>输出:</strong> [-1,-1]
<strong>解释:</strong> 链表 [2,3,3,2] 中不存在临界点。
</pre>

#### 提示:
* 链表中节点的数量在范围 <code>[2, 10<sup>5</sup>]</code> 内
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
    def nodesBetweenCriticalPoints(self, head: Optional[ListNode]) -> List[int]:
        prev = head
        curr = head.next
        firstlocal = -1
        prevlocal = -1
        i = 0
        ret = [-1, -1]

        while curr.next:
            if (prev.val < curr.val and curr.val > curr.next.val) \
                    or (prev.val > curr.val and curr.val < curr.next.val):
                if firstlocal == -1:
                    firstlocal = i
                elif ret[0] == -1:
                    ret = [i - prevlocal, i - firstlocal]
                else:
                    ret = [min(ret[0], i - prevlocal), i - firstlocal]
                prevlocal = i

            prev = curr
            curr = curr.next
            i += 1

        return ret
```
