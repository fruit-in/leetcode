# 817. 链表组件
给定链表头结点 `head`，该链表上的每个结点都有一个 **唯一的整型值** 。

同时给定列表 `G`，该列表是上述链表中整型值的一个子集。

返回列表 `G` 中组件的个数，这里对组件的定义为：链表中一段最长连续结点的值（该值必须在列表 `G` 中）构成的集合。

#### 示例 1:
<pre>
<strong>输入:</strong>
head: 0->1->2->3
G = [0, 1, 3]
<strong>输出:</strong> 2
<strong>解释:</strong>
链表中,0 和 1 是相连接的，且 G 中不包含 2，所以 [0, 1] 是 G 的一个组件，同理 [3] 也是一个组件，故返回 2。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong>
head: 0->1->2->3->4
G = [0, 3, 1, 4]
<strong>输出:</strong> 2
<strong>解释:</strong>
链表中，0 和 1 是相连接的，3 和 4 是相连接的，所以 [0, 1] 和 [3, 4] 是两个组件，故返回 2。
</pre>

#### 提示:
* 如果 `N` 是给定链表 `head` 的长度，`1 <= N <= 10000`。
* 链表中每个结点的值所在范围为 `[0, N - 1]`。
* `1 <= G.length <= 10000`
* `G` 是链表中所有结点的值的一个子集.

## 题解 (Python)

### 1. 题解
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def numComponents(self, head: ListNode, G: List[int]) -> int:
        G = set(G)
        curr = head
        ret = 0

        while curr:
            if curr.val in G and (not curr.next or curr.next.val not in G):
                ret += 1

            curr = curr.next

        return ret
```

## 题解 (Ruby)

### 1. 题解
```Ruby
# Definition for singly-linked list.
# class ListNode
#     attr_accessor :val, :next
#     def initialize(val = 0, _next = nil)
#         @val = val
#         @next = _next
#     end
# end
# @param {ListNode} head
# @param {Integer[]} g
# @return {Integer}
def num_components(head, g)
  g = g.to_set
  curr = head
  ret = 0

  until curr.nil?
    ret += 1 if g.member?(curr.val) && (curr.next.nil? || !g.member?(curr.next.val))

    curr = curr.next
  end

  ret
end
```
