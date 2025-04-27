# 460. LFU Cache
Design and implement a data structure for a [Least Frequently Used (LFU)](https://en.wikipedia.org/wiki/Least_frequently_used) cache.

Implement the `LFUCache` class:
* `LFUCache(int capacity)` Initializes the object with the `capacity` of the data structure.
* `int get(int key)` Gets the value of the `key` if the `key` exists in the cache. Otherwise, returns `-1`.
* `void put(int key, int value)` Update the value of the `key` if present, or inserts the `key` if not already present. When the cache reaches its `capacity`, it should invalidate and remove the **least frequently used** key before inserting a new item. For this problem, when there is a **tie** (i.e., two or more keys with the same frequency), the **least recently used** `key` would be invalidated.

To determine the least frequently used key, a **use counter** is maintained for each key in the cache. The key with the smallest **use counter** is the least frequently used key.

When a key is first inserted into the cache, its **use counter** is set to `1` (due to the `put` operation). The **use counter** for a key in the cache is incremented either a `get` or `put` operation is called on it.

The functions `get` and `put` must each run in `O(1)` average time complexity.

#### Example 1:
<pre>
<strong>Input:</strong>
["LFUCache", "put", "put", "get", "put", "get", "get", "put", "get", "get", "get"]
[[2], [1, 1], [2, 2], [1], [3, 3], [2], [3], [4, 4], [1], [3], [4]]
<strong>Output:</strong>
[null, null, null, 1, null, -1, 3, null, -1, 3, 4]
<strong>Explanation:</strong>
// cnt(x) = the use counter for key x
// cache=[] will show the last used order for tiebreakers (leftmost element is  most recent)
LFUCache lfu = new LFUCache(2);
lfu.put(1, 1);   // cache=[1,_], cnt(1)=1
lfu.put(2, 2);   // cache=[2,1], cnt(2)=1, cnt(1)=1
lfu.get(1);      // return 1
                 // cache=[1,2], cnt(2)=1, cnt(1)=2
lfu.put(3, 3);   // 2 is the LFU key because cnt(2)=1 is the smallest, invalidate 2.
                 // cache=[3,1], cnt(3)=1, cnt(1)=2
lfu.get(2);      // return -1 (not found)
lfu.get(3);      // return 3
                 // cache=[3,1], cnt(3)=2, cnt(1)=2
lfu.put(4, 4);   // Both 1 and 3 have the same cnt, but 1 is LRU, invalidate 1.
                 // cache=[4,3], cnt(4)=1, cnt(3)=2
lfu.get(1);      // return -1 (not found)
lfu.get(3);      // return 3
                 // cache=[3,4], cnt(4)=1, cnt(3)=3
lfu.get(4);      // return 4
                 // cache=[4,3], cnt(4)=2, cnt(3)=3
</pre>

#### Constraints:
* <code>1 <= capacity <= 10<sup>4</sup></code>
* <code>0 <= key <= 10<sup>5</sup></code>
* <code>0 <= value <= 10<sup>9</sup></code>
* At most <code>2 * 10<sup>5</sup></code> calls will be made to `get` and `put`.

## Solutions (Python)

### 1. Solution
```Python
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
```
