# 622. 设计循环队列
设计你的循环队列实现。 循环队列是一种线性数据结构，其操作表现基于 FIFO（先进先出）原则并且队尾被连接在队首之后以形成一个循环。它也被称为“环形缓冲器”。

循环队列的一个好处是我们可以利用这个队列之前用过的空间。在一个普通队列里，一旦一个队列满了，我们就不能插入下一个元素，即使在队列前面仍有空间。但是使用循环队列，我们能使用这些空间去存储新的值。

你的实现应该支持如下操作:
* <code>MyCircularQueue(k)</code>: 构造器，设置队列长度为 k 。
* <code>Front</code>: 从队首获取元素。如果队列为空，返回 -1 。
* <code>Rear</code>: 获取队尾元素。如果队列为空，返回 -1 。
* <code>enQueue(value)</code>: 向循环队列插入一个元素。如果成功插入则返回真。
* <code>deQueue()</code>: 从循环队列中删除一个元素。如果成功删除则返回真。
* <code>isEmpty()</code>: 检查循环队列是否为空。
* <code>isFull()</code>: 检查循环队列是否已满。

#### 示例:
<pre>
MyCircularQueue circularQueue = new MycircularQueue(3); // 设置长度为 3

circularQueue.enQueue(1);  // 返回 true

circularQueue.enQueue(2);  // 返回 true

circularQueue.enQueue(3);  // 返回 true

circularQueue.enQueue(4);  // 返回 false，队列已满

circularQueue.Rear();  // 返回 3

circularQueue.isFull();  // 返回 true

circularQueue.deQueue();  // 返回 true

circularQueue.enQueue(4);  // 返回 true

circularQueue.Rear();  // 返回 4
</pre>

#### 提示:
* 所有的值都在 0 至 1000 的范围内；
* 操作数将在 1 至 1000 的范围内；
* 请不要使用内置的队列库。

## 题解 (Rust)

### 1. 题解
```Rust
struct MyCircularQueue {
    data: Vec<i32>,
    size: usize,
    len: usize,
    head: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {

    /** Initialize your data structure here. Set the size of the queue to be k. */
    fn new(k: i32) -> Self {
        let k = k as usize;
        Self {
            data: vec![0; k],
            size: k,
            len: 0,
            head: 0,
        }
    }

    /** Insert an element into the circular queue. Return true if the operation is successful. */
    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.data[(self.head + self.len) % self.size] = value;
            self.len += 1;
            true
        }
    }

    /** Delete an element from the circular queue. Return true if the operation is successful. */
    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.head += 1;
            self.head %= self.size;
            self.len -= 1;
            true
        }
    }

    /** Get the front item from the queue. */
    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[self.head]
        }
    }

    /** Get the last item from the queue. */
    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.data[(self.head + self.len - 1) % self.size]
        }
    }

    /** Checks whether the circular queue is empty or not. */
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    /** Checks whether the circular queue is full or not. */
    fn is_full(&self) -> bool {
        self.len == self.size
    }
}

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */
```
