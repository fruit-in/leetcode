class ListNode:

    def __init__(self, key=-1, val=0, prev=None, next=None):
        self.key = key
        self.val = val
        self.prev = prev
        self.next = next
        self.count = 0


class LFUCache:

    def __init__(self, capacity: int):
        self.capacity = capacity
        self.cache = {}
        self.headtails = {}
        self.hair = ListNode()

    def get(self, key: int) -> int:
        if key not in self.cache:
            return -1

        node = self.cache[key]
        head, tail = self.headtails[node.count]
        if head.key == node.key and tail.key == node.key:
            self.headtails.pop(node.count)
        elif head.key == node.key:
            self.headtails[node.count][0] = node.next
        elif tail.key == node.key:
            self.headtails[node.count][1] = node.prev
        node.prev.next = node.next
        if node.next is not None:
            node.next.prev = node.prev

        node.count += 1
        if tail.next is None or tail.next.count > node.count:
            if tail.key == node.key:
                tail = node.prev
            self.headtails[node.count] = [node, node]
        else:
            tail = self.headtails[node.count][1]
            self.headtails[node.count][1] = node
        node.prev = tail
        node.next = tail.next
        tail.next = node
        if node.next is not None:
            node.next.prev = node

        return node.val

    def put(self, key: int, value: int) -> None:
        if key not in self.cache:
            if len(self.cache) == self.capacity:
                head, tail = self.headtails[self.hair.next.count]
                self.cache.pop(head.key)
                if head.key == tail.key:
                    self.headtails.pop(head.count)
                else:
                    self.headtails[head.count][0] = head.next
                self.hair.next = head.next
                if head.next is not None:
                    head.next.prev = self.hair

            node = ListNode(key, value, self.hair, self.hair.next)
            self.cache[key] = node
            self.headtails[0] = [node, node]
            self.hair.next = node
            if node.next is not None:
                node.next.prev = node

        self.cache[key].val = value
        self.get(key)


# Your LFUCache object will be instantiated and called as such:
# obj = LFUCache(capacity)
# param_1 = obj.get(key)
# obj.put(key,value)
