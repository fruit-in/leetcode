# 1019. 链表中的下一个更大节点
给出一个以头节点 `head` 作为第一个节点的链表。链表中的节点分别编号为：`node_1, node_2, node_3, ...` 。

每个节点都可能有下一个更大值（*next larger* **value**）：对于 `node_i`，如果其 `next_larger(node_i)` 是 `node_j.val`，那么就有 `j > i` 且  `node_j.val > node_i.val`，而 `j` 是可能的选项中最小的那个。如果不存在这样的 `j`，那么下一个更大值为 `0` 。

返回整数答案数组 `answer`，其中 `answer[i] = next_larger(node_{i+1})` 。

***注意:*** 在下面的示例中，诸如 `[2,1,5]` 这样的**输入**（不是输出）是链表的序列化表示，其头节点的值为 2，第二个节点值为 1，第三个节点值为 5 。

#### 示例 1:
<pre>
<strong>输入:</strong> [2,1,5]
<strong>输出:</strong> [5,5,0]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [2,7,4,3,5]
<strong>输出:</strong> [7,0,5,5,0]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> [1,7,5,1,9,2,5,1]
<strong>输出:</strong> [7,9,9,9,0,5,0,0]
</pre>

#### 提示:
1. 对于链表中的每个节点，`1 <= node.val <= 10^9`
2. 给定列表的长度在 `[0, 10000]` 范围内

## 题解 (Python)

### 1. 栈
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def nextLargerNodes(self, head: ListNode) -> List[int]:
        curr = head
        vals = []

        while curr is not None:
            vals.append(curr.val)
            curr = curr.next

        stack = []
        ret = [0] * len(vals)

        for i in range(len(vals) - 1, -1, -1):
            while stack != [] and vals[i] >= stack[-1]:
                stack.pop()
            if stack != []:
                ret[i] = stack[-1]
            stack.append(vals[i])

        return ret
```

## 题解 (Ruby)

### 1. 栈
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
# @return {Integer[]}
def next_larger_nodes(head)
  curr = head
  vals = []

  until curr.nil?
    vals.push(curr.val)
    curr = curr.next
  end

  stack = []
  ret = [0] * vals.size

  (vals.size - 1..0).step(-1).each do |i|
    stack.pop until stack.empty? || vals[i] < stack[-1]
    ret[i] = stack[-1] unless stack.empty?
    stack.push(vals[i])
  end

  ret
end
```
