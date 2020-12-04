# 82. 删除排序链表中的重复元素 II
给定一个排序链表，删除所有含有重复数字的节点，只保留原始链表中 *没有重复出现* 的数字。

#### 示例 1:
<pre>
<strong>输入:</strong> 1->2->3->3->4->4->5
<strong>输出:</strong> 1->2->5
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 1->1->1->2->3
<strong>输出:</strong> 2->3
</pre>

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
# @return {ListNode}
def delete_duplicates(head)
  dummy = ListNode.new(0, head)
  curr = dummy

  until curr.nil?
    if !curr.next.nil? && !curr.next.next.nil? && curr.next.val == curr.next.next.val
      curr.next.next = curr.next.next.next while !curr.next.next.nil? && curr.next.val == curr.next.next.val
      curr.next = curr.next.next
    else
      curr = curr.next
    end
  end

  dummy.next
end
```
