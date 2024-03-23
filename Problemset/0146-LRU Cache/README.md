# 146. LRU Cache
Design a data structure that follows the constraints of a [**Least Recently Used (LRU) cache**](https://en.wikipedia.org/wiki/Cache_replacement_policies#LRU).

Implement the `LRUCache` class:

* `LRUCache(int capacity)` Initialize the LRU cache with **positive** size `capacity`.
* `int get(int key)` Return the value of the `key` if the key exists, otherwise return `-1`.
* `void put(int key, int value)` Update the value of the `key` if the `key` exists. Otherwise, add the `key-value` pair to the cache. If the number of keys exceeds the `capacity` from this operation, **evict** the least recently used key.

The functions `get` and `put` must each run in `O(1)` average time complexity.

#### Example 1:
<pre>
<strong>Input:</strong>
["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
[[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
<strong>Output:</strong>
[null, null, null, 1, null, -1, null, -1, 3, 4]
<strong>Explanation:</strong>
LRUCache lRUCache = new LRUCache(2);
lRUCache.put(1, 1); // cache is {1=1}
lRUCache.put(2, 2); // cache is {1=1, 2=2}
lRUCache.get(1);    // return 1
lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
lRUCache.get(2);    // returns -1 (not found)
lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
lRUCache.get(1);    // return -1 (not found)
lRUCache.get(3);    // return 3
lRUCache.get(4);    // return 4
</pre>

#### Constraints:
* `1 <= capacity <= 3000`
* <code>0 <= key <= 104</sup></code>
* <code>0 <= value <= 105</sup></code>
* At most <code>2 * 10<sup>5</sup></code> calls will be made to `get` and `put`.

## Solutions (Python)

### 1. Solution
```Python
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
```
