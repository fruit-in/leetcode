# 146. LRU 缓存
请你设计并实现一个满足  [LRU (最近最少使用) 缓存](https://baike.baidu.com/item/LRU) 约束的数据结构。
实现 `LRUCache` 类：

* `LRUCache(int capacity)` 以 **正整数** 作为容量 `capacity` 初始化 LRU 缓存
* `int get(int key)` 如果关键字 `key` 存在于缓存中，则返回关键字的值，否则返回 `-1` 。
* `void put(int key, int value)` 如果关键字 `key` 已经存在，则变更其数据值 `value` ；如果不存在，则向缓存中插入该组 `key-value` 。如果插入操作导致关键字数量超过 `capacity` ，则应该 **逐出** 最久未使用的关键字。

函数 `get` 和 `put` 必须以 `O(1)` 的平均时间复杂度运行。

#### 示例 1:
<pre>
<strong>输入:</strong>
["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
[[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
<strong>输出:</strong>
[null, null, null, 1, null, -1, null, -1, 3, 4]
<strong>解释:</strong>
LRUCache lRUCache = new LRUCache(2);
lRUCache.put(1, 1); // 缓存是 {1=1}
lRUCache.put(2, 2); // 缓存是 {1=1, 2=2}
lRUCache.get(1);    // 返回 1
lRUCache.put(3, 3); // 该操作会使得关键字 2 作废，缓存是 {1=1, 3=3}
lRUCache.get(2);    // 返回 -1 (未找到)
lRUCache.put(4, 4); // 该操作会使得关键字 1 作废，缓存是 {4=4, 3=3}
lRUCache.get(1);    // 返回 -1 (未找到)
lRUCache.get(3);    // 返回 3
lRUCache.get(4);    // 返回 4
</pre>

#### 提示:
* `1 <= capacity <= 3000`
* <code>0 <= key <= 104</sup></code>
* <code>0 <= value <= 105</sup></code>
* 最多调用 <code>2 * 10<sup>5</sup></code> 次 `get` 和 `put`

## 题解 (Python)

### 1. 题解
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
