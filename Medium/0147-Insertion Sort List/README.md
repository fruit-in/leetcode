# 147. Insertion Sort List
Sort a linked list using insertion sort.

![](https://upload.wikimedia.org/wikipedia/commons/0/0f/Insertion-sort-example-300px.gif)<br>
A graphical example of insertion sort. The partial sorted list (black) initially contains only the first element in the list.<br>
With each iteration one element (red) is removed from the input data and inserted in-place into the sorted list

#### Algorithm of Insertion Sort:
1. Insertion sort iterates, consuming one input element each repetition, and growing a sorted output list.
2. At each iteration, insertion sort removes one element from the input data, finds the location it belongs within the sorted list, and inserts it there.
3. It repeats until no input elements remain.

#### Example 1:
<pre>
<strong>Input:</strong> 4->2->1->3
<strong>Output:</strong> 1->2->3->4
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> -1->5->3->4->0
<strong>Output:</strong> -1->0->3->4->5
</pre>

## Solutions (Python)

### 1. Solution
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
