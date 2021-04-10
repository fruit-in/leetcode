# 1721. 交换链表中的节点
给你链表的头节点 `head` 和一个整数 `k` 。

**交换** 链表正数第 `k` 个节点和倒数第 `k` 个节点的值后，返回链表的头节点（链表 **从 1 开始索引**）。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/01/10/linked1.jpg)
<pre>
<strong>输入:</strong> head = [1,2,3,4,5], k = 2
<strong>输出:</strong> [1,4,3,2,5]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> head = [7,9,6,6,7,8,3,0,9,5], k = 5
<strong>输出:</strong> [7,9,6,6,8,7,3,0,9,5]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> head = [1], k = 1
<strong>输出:</strong> [1]
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> head = [1,2], k = 1
<strong>输出:</strong> [2,1]
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> head = [1,2,3], k = 2
<strong>输出:</strong> [1,2,3]
</pre>

#### 提示:
* 链表中节点的数目是 `n`
* <code>1 <= k <= n <= 10<sup>5</sup></code>
* `0 <= Node.val <= 100`

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
# @param {Integer} k
# @return {ListNode}
def swap_nodes(head, k)
  dummy = ListNode.new(0, head)
  curr = head
  n = 0
  until curr.nil?
    n += 1
    curr = curr.next
  end

  curr = dummy
  node0 = nil
  node1 = nil
  (0..n).each do |i|
    node0 = curr if i == k - 1
    node1 = curr if i == n - k
    curr = curr.next
  end

  temp = node0.next
  node0.next = node1.next
  node1.next = temp
  temp = node0.next.next
  node0.next.next = node1.next.next
  node1.next.next = temp

  dummy.next
end
```
