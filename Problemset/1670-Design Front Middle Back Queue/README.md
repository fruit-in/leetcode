# 1670. Design Front Middle Back Queue
Design a queue that supports `push` and `pop` operations in the front, middle, and back.

Implement the `FrontMiddleBack` class:
* `FrontMiddleBack()` Initializes the queue.
* `void pushFront(int val)` Adds `val` to the **front** of the queue.
* `void pushMiddle(int val)` Adds `val` to the **middle** of the queue.
* `void pushBack(int val)` Adds `val` to the **back** of the queue.
* `int popFront()` Removes the **front** element of the queue and returns it. If the queue is empty, return `-1`.
* `int popMiddle()` Removes the **middle** element of the queue and returns it. If the queue is empty, return `-1`.
* `int popBack()` Removes the **back** element of the queue and returns it. If the queue is empty, return `-1`.

**Notice** that when there are **two** middle position choices, the operation is performed on the **frontmost** middle position choice. For example:
* Pushing `6` into the middle of `[1, 2, 3, 4, 5]` results in `[1, 2, 6, 3, 4, 5]`.
* Popping the middle from `[1, 2, 3, 4, 5, 6]` returns `3` and results in `[1, 2, 4, 5, 6]`.

#### Example 1:
<pre>
<strong>Input:</strong>
["FrontMiddleBackQueue", "pushFront", "pushBack", "pushMiddle", "pushMiddle", "popFront", "popMiddle", "popMiddle", "popBack", "popFront"]
[[], [1], [2], [3], [4], [], [], [], [], []]
<strong>Output:</strong>
[null, null, null, null, null, 1, 3, 4, 2, -1]
<strong>Explanation:</strong>
FrontMiddleBackQueue q = new FrontMiddleBackQueue();
q.pushFront(1);   // [1]
q.pushBack(2);    // [1, 2]
q.pushMiddle(3);  // [1, 3, 2]
q.pushMiddle(4);  // [1, 4, 3, 2]
q.popFront();     // return 1 -> [4, 3, 2]
q.popMiddle();    // return 3 -> [4, 2]
q.popMiddle();    // return 4 -> [2]
q.popBack();      // return 2 -> []
q.popFront();     // return -1 -> [] (The queue is empty)
</pre>

#### Constraints:
* <code>1 <= val <= 10<sup>9</sup></code>
* At most `1000` calls will be made to `pushFront`, `pushMiddle`, `pushBack`, `popFront`, `popMiddle`, and `popBack`.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::VecDeque;

struct FrontMiddleBackQueue {
    front_half: VecDeque<i32>,
    back_half: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrontMiddleBackQueue {
    fn new() -> Self {
        Self {
            front_half: VecDeque::new(),
            back_half: VecDeque::new(),
        }
    }

    fn push_front(&mut self, val: i32) {
        self.front_half.push_front(val);
        if self.front_half.len() > self.back_half.len() {
            self.back_half
                .push_front(self.front_half.pop_back().unwrap());
        }
    }

    fn push_middle(&mut self, val: i32) {
        if self.front_half.len() < self.back_half.len() {
            self.front_half.push_back(val);
        } else {
            self.back_half.push_front(val);
        }
    }

    fn push_back(&mut self, val: i32) {
        self.back_half.push_back(val);
        if self.front_half.len() + 1 < self.back_half.len() {
            self.front_half
                .push_back(self.back_half.pop_front().unwrap());
        }
    }

    fn pop_front(&mut self) -> i32 {
        if self.front_half.len() < self.back_half.len() {
            self.front_half
                .push_back(self.back_half.pop_front().unwrap());
        }
        self.front_half.pop_front().unwrap_or(-1)
    }

    fn pop_middle(&mut self) -> i32 {
        if self.front_half.len() < self.back_half.len() {
            self.back_half.pop_front().unwrap_or(-1)
        } else {
            self.front_half.pop_back().unwrap_or(-1)
        }
    }

    fn pop_back(&mut self) -> i32 {
        if self.front_half.len() == self.back_half.len() {
            self.back_half
                .push_front(self.front_half.pop_back().unwrap_or(-1));
        }
        self.back_half.pop_back().unwrap()
    }
}

/**
 * Your FrontMiddleBackQueue object will be instantiated and called as such:
 * let obj = FrontMiddleBackQueue::new();
 * obj.push_front(val);
 * obj.push_middle(val);
 * obj.push_back(val);
 * let ret_4: i32 = obj.pop_front();
 * let ret_5: i32 = obj.pop_middle();
 * let ret_6: i32 = obj.pop_back();
 */
```
