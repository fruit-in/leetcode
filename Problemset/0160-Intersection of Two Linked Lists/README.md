# 160. Intersection of Two Linked Lists
Write a program to find the node at which the intersection of two singly linked lists begins.

For example, the following two linked lists:
![](https://assets.leetcode.com/uploads/2018/12/13/160_statement.png)<br>
begin to intersect at node c1.

#### Example 1:
![](https://assets.leetcode.com/uploads/2018/12/13/160_example_1.png)
<pre>
<strong>Input:</strong> intersectVal = 8, listA = [4,1,8,4,5], listB = [5,0,1,8,4,5], skipA = 2, skipB = 3
<strong>Output:</strong> Reference of the node with value = 8
<strong>Input Explanation:</strong> The intersected node's value is 8 (note that this must not be 0 if the two lists intersect).
                   From the head of A, it reads as [4,1,8,4,5].
                   From the head of B, it reads as [5,0,1,8,4,5].
                   There are 2 nodes before the intersected node in A;
                   There are 3 nodes before the intersected node in B.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2018/12/13/160_example_2.png)
<pre>
<strong>Input:</strong> intersectVal = 2, listA = [0,9,1,2,4], listB = [3,2,4], skipA = 3, skipB = 1
<strong>Output:</strong> Reference of the node with value = 2
<strong>Input Explanation:</strong> The intersected node's value is 2 (note that this must not be 0 if the two lists intersect).
                   From the head of A, it reads as [0,9,1,2,4].
                   From the head of B, it reads as [3,2,4].
                   There are 3 nodes before the intersected node in A;
                   There are 1 node before the intersected node in B.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2018/12/13/160_example_3.png)
<pre>
<strong>Input:</strong> intersectVal = 0, listA = [2,6,4], listB = [1,5], skipA = 3, skipB = 2
<strong>Output:</strong> null
<strong>Input Explanation:</strong> From the head of A, it reads as [2,6,4].
                   From the head of B, it reads as [1,5].
                   Since the two lists do not intersect, intersectVal must be 0, while skipA and skipB can be arbitrary values.
<strong>Explanation:</strong> The two lists do not intersect, so return null.
</pre>

#### Notes:
* If the two linked lists have no intersection at all, return ```null```.
* The linked lists must retain their original structure after the function returns.
* You may assume there are no cycles anywhere in the entire linked structure.
* Your code should preferably run in O(n) time and use only O(1) memory.

## Solutions (Python)

### 1. Set
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

### 2. Change Head to Make Same Lengths
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

### 3. Two Pointers
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
