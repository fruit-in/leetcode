# 641. 设计循环双端队列
设计实现双端队列。

实现 `MyCircularDeque` 类:

* `MyCircularDeque(int k)` ：构造函数,双端队列最大为 `k` 。
* `boolean insertFront()`：将一个元素添加到双端队列头部。 如果操作成功返回 `true` ，否则返回 `false` 。
* `boolean insertLast()` ：将一个元素添加到双端队列尾部。如果操作成功返回 `true` ，否则返回 `false` 。
* `boolean deleteFront()` ：从双端队列头部删除一个元素。 如果操作成功返回 `true` ，否则返回 `false` 。
* `boolean deleteLast()` ：从双端队列尾部删除一个元素。如果操作成功返回 `true` ，否则返回 `false` 。
* `int getFront()`：从双端队列头部获得一个元素。如果双端队列为空，返回 `-1` 。
* `int getRear()` ：获得双端队列的最后一个元素。 如果双端队列为空，返回 `-1` 。
* `boolean isEmpty()` ：若双端队列为空，则返回 `true` ，否则返回 `false`  。
* `boolean isFull()` ：若双端队列满了，则返回 `true` ，否则返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong>
["MyCircularDeque", "insertLast", "insertLast", "insertFront", "insertFront", "getRear", "isFull", "deleteLast", "insertFront", "getFront"]
[[3], [1], [2], [3], [4], [], [], [], [4], []]
<strong>输出:</strong>
[null, true, true, true, false, 2, true, true, true, 4]
<strong>解释:</strong>
MyCircularDeque circularDeque = new MycircularDeque(3); // 设置容量大小为3
circularDeque.insertLast(1);                    // 返回 true
circularDeque.insertLast(2);                    // 返回 true
circularDeque.insertFront(3);                   // 返回 true
circularDeque.insertFront(4);                   // 已经满了，返回 false
circularDeque.getRear();                // 返回 2
circularDeque.isFull();                     // 返回 true
circularDeque.deleteLast();                 // 返回 true
circularDeque.insertFront(4);                   // 返回 true
circularDeque.getFront();               // 返回 4
</pre>

#### 提示:
* `1 <= k <= 1000`
* `0 <= value <= 1000`
* `insertFront`, `insertLast`, `deleteFront`, `deleteLast`, `getFront`, `getRear`, `isEmpty`, `isFull`  调用次数不大于 `2000` 次

## 题解 (Rust)

### 1. 题解
```Rust
struct MyCircularDeque {
    deque: Vec<i32>,
    size: usize,
    front: usize,
    back: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            deque: vec![0; k as usize],
            size: 0,
            front: 0,
            back: 0,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        if !self.is_empty() {
            self.front = (self.front + self.deque.len() - 1) % self.deque.len();
        }
        self.deque[self.front] = value;
        self.size += 1;

        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        if !self.is_empty() {
            self.back = (self.back + 1) % self.deque.len();
        }
        self.deque[self.back] = value;
        self.size += 1;

        true
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        if self.size > 1 {
            self.front = (self.front + 1) % self.deque.len();
        }
        self.size -= 1;

        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }

        if self.size > 1 {
            self.back = (self.back + self.deque.len() - 1) % self.deque.len();
        }
        self.size -= 1;

        true
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }

        self.deque[self.front]
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }

        self.deque[self.back]
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.deque.len()
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */
```
