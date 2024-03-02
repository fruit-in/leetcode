# 641. Design Circular Deque
Design your implementation of the circular double-ended queue (deque).

Implement the `MyCircularDeque` class:

* `MyCircularDeque(int k)` Initializes the deque with a maximum size of `k`.
* `boolean insertFront()` Adds an item at the front of Deque. Returns `true` if the operation is successful, or `false` otherwise.
* `boolean insertLast()` Adds an item at the rear of Deque. Returns `true` if the operation is successful, or `false` otherwise.
* `boolean deleteFront()` Deletes an item from the front of Deque. Returns `true` if the operation is successful, or `false` otherwise.
* `boolean deleteLast()` Deletes an item from the rear of Deque. Returns `true` if the operation is successful, or `false` otherwise.
* `int getFront()` Returns the front item from the Deque. Returns `-1` if the deque is empty.
* `int getRear()` Returns the last item from Deque. Returns `-1` if the deque is empty.
* `boolean isEmpty()` Returns `true` if the deque is empty, or `false` otherwise.
* `boolean isFull()` Returns `true` if the deque is full, or `false` otherwise.

#### Example 1:
<pre>
<strong>Input:</strong>
["MyCircularDeque", "insertLast", "insertLast", "insertFront", "insertFront", "getRear", "isFull", "deleteLast", "insertFront", "getFront"]
[[3], [1], [2], [3], [4], [], [], [], [4], []]
<strong>Output:</strong>
[null, true, true, true, false, 2, true, true, true, 4]
<strong>Explanation:</strong>
MyCircularDeque myCircularDeque = new MyCircularDeque(3);
myCircularDeque.insertLast(1);  // return True
myCircularDeque.insertLast(2);  // return True
myCircularDeque.insertFront(3); // return True
myCircularDeque.insertFront(4); // return False, the queue is full.
myCircularDeque.getRear();      // return 2
myCircularDeque.isFull();       // return True
myCircularDeque.deleteLast();   // return True
myCircularDeque.insertFront(4); // return True
myCircularDeque.getFront();     // return 4
</pre>

#### Constraints:
* `1 <= k <= 1000`
* `0 <= value <= 1000`
* At most `2000` calls will be made to `insertFront`, `insertLast`, `deleteFront`, `deleteLast`, `getFront`, `getRear`, `isEmpty`, `isFull`.

## Solutions (Rust)

### 1. Solution
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
