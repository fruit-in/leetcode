# 160. 相交链表
编写一个程序，找到两个单链表相交的起始节点。

如下面的两个链表：

![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/12/14/160_statement.png)

在节点 c1 开始相交。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2018/12/13/160_example_1.png)
<pre>
<strong>输入:</strong> intersectVal = 8, listA = [4,1,8,4,5], listB = [5,0,1,8,4,5], skipA = 2, skipB = 3
<strong>输出:</strong> Reference of the node with value = 8
<strong>输入解释:</strong> 相交节点的值为 8 （注意，如果两个列表相交则不能为 0）。
从各自的表头开始算起，链表 A 为 [4,1,8,4,5]，链表 B 为 [5,0,1,8,4,5]。
在 A 中，相交节点前有 2 个节点；在 B 中，相交节点前有 3 个节点。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2018/12/13/160_example_2.png)
<pre>
<strong>输入:</strong> intersectVal = 2, listA = [0,9,1,2,4], listB = [3,2,4], skipA = 3, skipB = 1
<strong>输出:</strong> Reference of the node with value = 2
<strong>输入解释:</strong> 相交节点的值为 2 （注意，如果两个列表相交则不能为 0）。
从各自的表头开始算起，链表 A 为 [0,9,1,2,4]，链表 B 为 [3,2,4]。
在 A 中，相交节点前有 3 个节点；在 B 中，相交节点前有 1 个节点。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2018/12/13/160_example_3.png)
<pre>
<strong>输入:</strong> intersectVal = 0, listA = [2,6,4], listB = [1,5], skipA = 3, skipB = 2
<strong>输出:</strong> null
<strong>输入解释:</strong> 从各自的表头开始算起，链表 A 为 [2,6,4]，链表 B 为 [1,5]。
由于这两个链表不相交，所以 intersectVal 必须为 0，而 skipA 和 skipB 可以是任意值。
<strong>解释:</strong> 这两个链表不相交，因此返回 null。
</pre>

#### 注意:
* 如果两个链表没有交点，返回 ```null```.
* 在返回结果后，两个链表仍须保持原有的结构。
* 可假定整个链表结构中没有循环。
* 程序尽量满足 O(*n*) 时间复杂度，且仅用 O(*1*) 内存。

## 题解 (Python)

### 1. 集合
```Python
# Definition for singly-linked list.
# class ListNode(object):
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution(object):
    def getIntersectionNode(self, headA, headB):
        """
        :type head1, head1: ListNode
        :rtype: ListNode
        """
        nodes = set()
        while headA:
            nodes.add(headA)
            headA = headA.next
        while headB:
            if headB in nodes:
                return headB
            headB = headB.next
        return None
```

### 2. 改变表头使长度相等
```Python
# Definition for singly-linked list.
# class ListNode(object):
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution(object):
    def getIntersectionNode(self, headA, headB):
        """
        :type head1, head1: ListNode
        :rtype: ListNode
        """
        cnt = 0
        curr = headA
        while curr:
            cnt += 1
            curr = curr.next

        curr = headB
        while curr:
            cnt -= 1
            curr = curr.next

        for _ in range(cnt):
            headA = headA.next
        for _ in range(-cnt):
            headB = headB.next

        while headA != headB:
            headA = headA.next
            headB = headB.next

        return headA
```

### 3. 双指针
```Python
# Definition for singly-linked list.
# class ListNode(object):
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution(object):
    def getIntersectionNode(self, headA, headB):
        """
        :type head1, head1: ListNode
        :rtype: ListNode
        """
        currA = headA
        currB = headB
        while currA != currB:
            currA = currA.next if currA else headB
            currB = currB.next if currB else headA
        return currA
```
