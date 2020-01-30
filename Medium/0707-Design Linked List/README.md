# 707. Design Linked List
Design your implementation of the linked list. You can choose to use the singly linked list or the doubly linked list. A node in a singly linked list should have two attributes: ```val``` and ```next```. ```val``` is the value of the current node, and ```next``` is a pointer/reference to the next node. If you want to use the doubly linked list, you will need one more attribute ```prev``` to indicate the previous node in the linked list. Assume all nodes in the linked list are 0-indexed.

Implement these functions in your linked list class:
* ```get(index)``` : Get the value of the ```index```-th node in the linked list. If the index is invalid, return ```-1```.
* ```addAtHead(val)``` : Add a node of value ```val``` before the first element of the linked list. After the insertion, the new node will be the first node of the linked list.
* ```addAtTail(val)``` : Append a node of value ```val``` to the last element of the linked list.
* ```addAtIndex(index, val)``` : Add a node of value ```val``` before the ```index```-th node in the linked list. If ```index``` equals to the length of linked list, the node will be appended to the end of linked list. If index is greater than the length, the node will not be inserted.
* ```deleteAtIndex(index)``` : Delete the ```index```-th node in the linked list, if the index is valid.

#### Example:
<pre>
<strong>Input:</strong>
["MyLinkedList","addAtHead","addAtTail","addAtIndex","get","deleteAtIndex","get"]
[[],[1],[3],[1,2],[1],[1],[1]]
<strong>Output:</strong>
[null,null,null,null,2,null,3]
<strong>Explanation:</strong>
MyLinkedList linkedList = new MyLinkedList(); // Initialize empty LinkedList
linkedList.addAtHead(1);
linkedList.addAtTail(3);
linkedList.addAtIndex(1, 2);  // linked list becomes 1->2->3
linkedList.get(1);            // returns 2
linkedList.deleteAtIndex(1);  // now the linked list is 1->3
linkedList.get(1);            // returns 3
</pre>

#### Constraints:
* ```0 <= index,val <= 1000```
* Please do not use the built-in LinkedList library.
* At most ```2000``` calls will be made to ```get```, ```addAtHead```, ```addAtTail```,  ```addAtIndex``` and ```deleteAtIndex```.

## Solutions (Python)

### 1. Singly Linked List
```Python
class ListNode:
    def __init__(self, val: int):
        self.val = val
        self.next = None


class MyLinkedList:
    def __init__(self):
        """
        Initialize your data structure here.
        """
        self.len = 0
        self.head = None


    def get(self, index: int) -> int:
        """
        Get the value of the index-th node in the linked list. If the index is invalid, return -1.
        """
        if index >= self.len:
            return -1
        curr = self.head
        for _ in range(index):
            curr = curr.next
        return curr.val


    def addAtHead(self, val: int) -> None:
        """
        Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list.
        """
        head = ListNode(val)
        head.next = self.head
        self.head = head
        self.len += 1


    def addAtTail(self, val: int) -> None:
        """
        Append a node of value val to the last element of the linked list.
        """
        self.addAtIndex(self.len, val)


    def addAtIndex(self, index: int, val: int) -> None:
        """
        Add a node of value val before the index-th node in the linked list. If index equals to the length of linked list, the node will be appended to the end of linked list. If index is greater than the length, the node will not be inserted.
        """
        if index == 0:
            self.addAtHead(val)
        elif index <= self.len:
            node = ListNode(val)
            curr = self.head
            for _ in range(index - 1):
                curr = curr.next
            node.next = curr.next
            curr.next = node
            self.len += 1


    def deleteAtIndex(self, index: int) -> None:
        """
        Delete the index-th node in the linked list, if the index is valid.
        """
        if index == 0 and self.len > 0:
            self.head = self.head.next
            self.len -= 1
        elif index < self.len:
            curr = self.head
            for _ in range(index - 1):
                curr = curr.next
            curr.next = curr.next.next
            self.len -= 1



# Your MyLinkedList object will be instantiated and called as such:
# obj = MyLinkedList()
# param_1 = obj.get(index)
# obj.addAtHead(val)
# obj.addAtTail(val)
# obj.addAtIndex(index,val)
# obj.deleteAtIndex(index)
```
