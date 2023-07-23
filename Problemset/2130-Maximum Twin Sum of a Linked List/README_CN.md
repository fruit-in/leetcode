# 2130. 链表最大孪生和
在一个大小为 `n` 且 `n` 为 **偶数** 的链表中，对于 `0 <= i <= (n / 2) - 1` 的 `i` ，第 `i` 个节点（下标从 **0** 开始）的孪生节点为第 `(n-1-i)` 个节点 。
* 比方说，`n = 4` 那么节点 `0` 是节点 `3` 的孪生节点，节点 `1` 是节点 `2` 的孪生节点。这是长度为 `n = 4` 的链表中所有的孪生节点。

**孪生和** 定义为一个节点和它孪生节点两者值之和。

给你一个长度为偶数的链表的头节点 `head` ，请你返回链表的 **最大孪生和** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/12/03/eg1drawio.png)
<pre>
<strong>输入:</strong> head = [5,4,2,1]
<strong>输出:</strong> 6
<strong>解释:</strong>
节点 0 和节点 1 分别是节点 3 和 2 的孪生节点。孪生和都为 6 。
链表中没有其他孪生节点。
所以，链表的最大孪生和是 6 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/12/03/eg2drawio.png)
<pre>
<strong>输入:</strong> head = [4,2,2,3]
<strong>输出:</strong> 7
<strong>解释:</strong>
链表中的孪生节点为：
- 节点 0 是节点 3 的孪生节点，孪生和为 4 + 3 = 7 。
- 节点 1 是节点 2 的孪生节点，孪生和为 2 + 2 = 4 。
所以，最大孪生和为 max(7, 4) = 7 。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2021/12/03/eg3drawio.png)
<pre>
<strong>输入:</strong> head = [1,100000]
<strong>输出:</strong> 100001
<strong>解释:</strong>
链表中只有一对孪生节点，孪生和为 1 + 100000 = 100001 。
</pre>

#### 提示:
* 链表的节点数目是 <code>[2, 10<sup>5</sup>]</code> 中的 **偶数** 。
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
    def pairSum(self, head: Optional[ListNode]) -> int:
        curr = head
        vals = []

        while curr:
            vals.append(curr.val)
            curr = curr.next

        return max(vals[i] + vals[-1 - i] for i in range(len(vals) // 2))
```
