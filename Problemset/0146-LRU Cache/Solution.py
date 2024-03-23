class ListNode:
    def __init__(self, key=0, val=0, prev=None, next=None):
        self.key = key
        self.val = val
        self.prev = prev
        self.next = next


class LRUCache:

    def __init__(self, capacity: int):
        self.capacity = capacity
        self.keynodes = {}
        self.head = None
        self.tail = None

    def get(self, key: int) -> int:
        if key not in self.keynodes:
            return -1

        self.__move2tail(key)

        return self.keynodes[key].val

    def put(self, key: int, value: int) -> None:
        if len(self.keynodes) == 0:
            self.head = ListNode(key, value)
            self.tail = self.head
            self.keynodes[key] = self.head
        elif key not in self.keynodes:
            self.tail.next = ListNode(key, value, self.tail)
            self.tail = self.tail.next
            self.keynodes[key] = self.tail
            if len(self.keynodes) > self.capacity:
                self.keynodes.pop(self.head.key)
                self.head = self.head.next
                self.head.prev = None
        else:
            self.keynodes[key].val = value
            self.__move2tail(key)

    def __move2tail(self, key: int) -> None:
        if len(self.keynodes) <= 1 or key not in self.keynodes or key == self.tail.key:
            pass
        elif key == self.head.key:
            self.head.next.prev = None
            self.tail.next = self.head
            self.head.prev = self.tail
            self.tail = self.head
            self.head = self.head.next
            self.tail.next = None
        else:
            node = self.keynodes[key]
            node.prev.next = node.next
            node.next.prev = node.prev
            node.prev = self.tail
            self.tail.next = node
            self.tail = node


# Your LRUCache object will be instantiated and called as such:
# obj = LRUCache(capacity)
# param_1 = obj.get(key)
# obj.put(key,value)
