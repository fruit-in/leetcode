# 1670. 设计前中后队列
请你设计一个队列，支持在前，中，后三个位置的 `push` 和 `pop` 操作。

请你完成 `FrontMiddleBack` 类：
* `FrontMiddleBack()` 初始化队列。
* `void pushFront(int val)` 将 `val` 添加到队列的 **最前面** 。
* `void pushMiddle(int val)` 将 `val` 添加到队列的 **正中间** 。
* `void pushBack(int val)` 将 `val` 添加到队里的 **最后面** 。
* `int popFront()` 将 **最前面** 的元素从队列中删除并返回值，如果删除之前队列为空，那么返回 `-1` 。
* `int popMiddle()` 将 **正中间** 的元素从队列中删除并返回值，如果删除之前队列为空，那么返回 `-1` 。
* `int popBack()` 将 **最后面** 的元素从队列中删除并返回值，如果删除之前队列为空，那么返回 `-1` 。

请注意当有 **两个** 中间位置的时候，选择靠前面的位置进行操作。比方说：
* 将 `6` 添加到 `[1, 2, 3, 4, 5]` 的中间位置，结果数组为 `[1, 2, 6, 3, 4, 5]` 。
* 从 `[1, 2, 3, 4, 5, 6]` 的中间位置弹出元素，返回 `3` ，数组变为 `[1, 2, 4, 5, 6]` 。

#### 示例 1:
<pre>
<strong>输入:</strong>
["FrontMiddleBackQueue", "pushFront", "pushBack", "pushMiddle", "pushMiddle", "popFront", "popMiddle", "popMiddle", "popBack", "popFront"]
[[], [1], [2], [3], [4], [], [], [], [], []]
<strong>输出:</strong>
[null, null, null, null, null, 1, 3, 4, 2, -1]
<strong>解释:</strong>
FrontMiddleBackQueue q = new FrontMiddleBackQueue();
q.pushFront(1);   // [1]
q.pushBack(2);    // [1, 2]
q.pushMiddle(3);  // [1, 3, 2]
q.pushMiddle(4);  // [1, 4, 3, 2]
q.popFront();     // 返回 1 -> [4, 3, 2]
q.popMiddle();    // 返回 3 -> [4, 2]
q.popMiddle();    // 返回 4 -> [2]
q.popBack();      // 返回 2 -> []
q.popFront();     // 返回 -1 -> [] （队列为空）
</pre>

#### 提示:
* <code>1 <= val <= 10<sup>9</sup></code>
* 最多调用 `1000` 次 `pushFront`， `pushMiddle`， `pushBack`， `popFront`， `popMiddle` 和 `popBack` 。

## 题解 (Rust)

### 1. 题解
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
