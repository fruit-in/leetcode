# 1669. 合并两个链表
给你两个链表 `list1` 和 `list2` ，它们包含的元素分别为 `n` 个和 `m` 个。

请你将 `list1` 中第 `a` 个节点到第 `b` 个节点删除，并将`list2` 接在被删除节点的位置。

下图中蓝色边和节点展示了操作后的结果：<br>
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/11/28/fig1.png)<br>
请你返回结果链表的头指针。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/11/28/merge_linked_list_ex1.png)
<pre>
<strong>输入:</strong> list1 = [0,1,2,3,4,5], a = 3, b = 4, list2 = [1000000,1000001,1000002]
<strong>输出:</strong> [0,1,2,1000000,1000001,1000002,5]
<strong>解释:</strong> 我们删除 list1 中第三和第四个节点，并将 list2 接在该位置。上图中蓝色的边和节点为答案链表。
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/11/28/merge_linked_list_ex2.png)
<pre>
<strong>输入:</strong> list1 = [0,1,2,3,4,5,6], a = 2, b = 5, list2 = [1000000,1000001,1000002,1000003,1000004]
<strong>输出:</strong> [0,1,1000000,1000001,1000002,1000003,1000004,6]
<strong>解释:</strong> 上图中蓝色的边和节点为答案链表。
</pre>

#### 提示:
* <code>3 <= list1.length <= 10<sup>4</sup></code>
* `1 <= a <= b < list1.length - 1`
* <code>1 <= list2.length <= 10<sup>4</sup></code>

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
# @param {ListNode} list1
# @param {Integer} a
# @param {Integer} b
# @param {ListNode} list2
# @return {ListNode}
def merge_in_between(list1, a, b, list2)
  removed_a = list1
  (1...a).each do |_|
    removed_a = removed_a.next
  end
  removed_b = removed_a.next
  (0..(b - a)).each do |_|
    removed_b = removed_b.next
  end

  curr = list2
  curr = curr.next until curr.next.nil?

  removed_a.next = list2
  curr.next = removed_b

  list1
end
```
