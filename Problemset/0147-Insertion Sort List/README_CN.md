# 147. 对链表进行插入排序
对链表进行插入排序。

![](https://upload.wikimedia.org/wikipedia/commons/0/0f/Insertion-sort-example-300px.gif)<br>
插入排序的动画演示如上。从第一个元素开始，该链表可以被认为已经部分排序（用黑色表示）。<br>
每次迭代时，从输入数据中移除一个元素（用红色表示），并原地将其插入到已排好序的链表中。

#### 插入排序算法:
1. 插入排序是迭代的，每次只移动一个元素，直到所有元素可以形成一个有序的输出列表。
2. 每次迭代中，插入排序只从输入数据中移除一个待排序的元素，找到它在序列中适当的位置，并将其插入。
3. 重复直到所有输入数据插入完为止。

#### 示例 1:
<pre>
<strong>输入:</strong> 4->2->1->3
<strong>输出:</strong> 1->2->3->4
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> -1->5->3->4->0
<strong>输出:</strong> -1->0->3->4->5
</pre>

## 题解 (Python)

### 1. 题解
```Python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def insertionSortList(self, head: ListNode) -> ListNode:
        if not head:
            return head

        dummy = ListNode(next=head)
        tail = head
        while tail.next and tail.val <= tail.next.val:
            tail = tail.next

        while tail.next:
            curr = tail.next
            tail.next = curr.next
            while tail.next and tail.val <= tail.next.val:
                tail = tail.next

            node = dummy

            while curr.val >= node.next.val:
                node = node.next

            curr.next = node.next
            node.next = curr

        return dummy.next
```
