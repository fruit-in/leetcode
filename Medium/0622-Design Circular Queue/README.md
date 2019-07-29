# 622. Design Circular Queue
Design your implementation of the circular queue. The circular queue is a linear data structure in which the operations are performed based on FIFO (First In First Out) principle and the last position is connected back to the first position to make a circle. It is also called "Ring Buffer".

One of the benefits of the circular queue is that we can make use of the spaces in front of the queue. In a normal queue, once the queue becomes full, we cannot insert the next element even if there is a space in front of the queue. But using the circular queue, we can use the space to store new values.

Your implementation should support following operations:
* <code>MyCircularQueue(k)</code>: Constructor, set the size of the queue to be k.
* <code>Front</code>: Get the front item from the queue. If the queue is empty, return -1.
* <code>Rear</code>: Get the last item from the queue. If the queue is empty, return -1.
* <code>enQueue(value)</code>: Insert an element into the circular queue. Return true if the operation is successful.
* <code>deQueue()</code>: Delete an element from the circular queue. Return true if the operation is successful.
* <code>isEmpty()</code>: Checks whether the circular queue is empty or not.
* <code>isFull()</code>: Checks whether the circular queue is full or not.

#### Example:
<pre>
MyCircularQueue circularQueue = new MyCircularQueue(3); // set the size to be 3
circularQueue.enQueue(1);  // return true
circularQueue.enQueue(2);  // return true
circularQueue.enQueue(3);  // return true
circularQueue.enQueue(4);  // return false, the queue is full
circularQueue.Rear();  // return 3
circularQueue.isFull();  // return true
circularQueue.deQueue();  // return true
circularQueue.enQueue(4);  // return true
circularQueue.Rear();  // return 4
</pre>

#### Note:
* All values will be in the range of [0, 1000].
* The number of operations will be in the range of [1, 1000].
* Please do not use the built-in Queue library.

## Solutions

### 1. Solution (Rust)
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
