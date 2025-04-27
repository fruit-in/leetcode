# 460. LFU 缓存
请你为 [最不经常使用（LFU）](https://baike.baidu.com/item/%E7%BC%93%E5%AD%98%E7%AE%97%E6%B3%95)缓存算法设计并实现数据结构。

实现 `LFUCache` 类：
* `LFUCache(int capacity)` - 用数据结构的容量 `capacity` 初始化对象
* `int get(int key)` - 如果键 `key` 存在于缓存中，则获取键的值，否则返回 `-1` 。
* `void put(int key, int value)` - 如果键 `key` 已存在，则变更其值；如果键不存在，请插入键值对。当缓存达到其容量 `capacity` 时，则应该在插入新项之前，移除最不经常使用的项。在此问题中，当存在平局（即两个或更多个键具有相同使用频率）时，应该去除 **最久未使用** 的键。

为了确定最不常使用的键，可以为缓存中的每个键维护一个 **使用计数器** 。使用计数最小的键是最久未使用的键。

当一个键首次插入到缓存中时，它的使用计数器被设置为 `1` (由于 put 操作)。对缓存中的键执行 `get` 或 `put` 操作，使用计数器的值将会递增。

函数 `get` 和 `put` 必须以 `O(1)` 的平均时间复杂度运行。

#### 示例:
<pre>
<strong>输入:</strong>
["LFUCache", "put", "put", "get", "put", "get", "get", "put", "get", "get", "get"]
[[2], [1, 1], [2, 2], [1], [3, 3], [2], [3], [4, 4], [1], [3], [4]]
<strong>输出:</strong>
[null, null, null, 1, null, -1, 3, null, -1, 3, 4]
<strong>解释:</strong>
// cnt(x) = 键 x 的使用计数
// cache=[] 将显示最后一次使用的顺序（最左边的元素是最近的）
LFUCache lfu = new LFUCache(2);
lfu.put(1, 1);   // cache=[1,_], cnt(1)=1
lfu.put(2, 2);   // cache=[2,1], cnt(2)=1, cnt(1)=1
lfu.get(1);      // 返回 1
                 // cache=[1,2], cnt(2)=1, cnt(1)=2
lfu.put(3, 3);   // 去除键 2 ，因为 cnt(2)=1 ，使用计数最小
                 // cache=[3,1], cnt(3)=1, cnt(1)=2
lfu.get(2);      // 返回 -1（未找到）
lfu.get(3);      // 返回 3
                 // cache=[3,1], cnt(3)=2, cnt(1)=2
lfu.put(4, 4);   // 去除键 1 ，1 和 3 的 cnt 相同，但 1 最久未使用
                 // cache=[4,3], cnt(4)=1, cnt(3)=2
lfu.get(1);      // 返回 -1（未找到）
lfu.get(3);      // 返回 3
                 // cache=[3,4], cnt(4)=1, cnt(3)=3
lfu.get(4);      // 返回 4
                 // cache=[3,4], cnt(4)=2, cnt(3)=3
</pre>

#### 提示:
* <code>1 <= capacity <= 10<sup>4</sup></code>
* <code>0 <= key <= 10<sup>5</sup></code>
* <code>0 <= value <= 10<sup>9</sup></code>
* 最多调用 <code>2 * 10<sup>5</sup></code> 次 `get` 和 `put` 方法

## 题解 (Python)

### 1. 题解
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
