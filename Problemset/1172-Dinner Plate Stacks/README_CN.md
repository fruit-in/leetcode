# 1172. 餐盘栈
我们把无限数量 ∞ 的栈排成一行，按从左到右的次序从 0 开始编号。每个栈的的最大容量 `capacity` 都相同。

实现一个叫「餐盘」的类 `DinnerPlates`：

* `DinnerPlates(int capacity)` - 给出栈的最大容量 `capacity`。
* `void push(int val)` - 将给出的正整数 `val` 推入 **从左往右第一个** 没有满的栈。
* `int pop()` - 返回 **从右往左第一个** 非空栈顶部的值，并将其从栈中删除；如果所有的栈都是空的，请返回 `-1`。
* `int popAtStack(int index)` - 返回编号 `index` 的栈顶部的值，并将其从栈中删除；如果编号 `index` 的栈是空的，请返回 `-1`。

#### 示例:
<pre>
<strong>输入:</strong>
["DinnerPlates", "push", "push", "push", "push", "push", "popAtStack", "push", "push", "popAtStack", "popAtStack", "pop", "pop", "pop", "pop", "pop"]
[[2], [1], [2], [3], [4], [5], [0], [20], [21], [0], [2], [], [], [], [], []]
<strong>输出:</strong>
[null, null, null, null, null, null, 2, null, null, 20, 21, 5, 4, 3, 1, -1]
<strong>解释:</strong>
DinnerPlates D = DinnerPlates(2);  // 初始化，栈最大容量 capacity = 2
D.push(1);
D.push(2);
D.push(3);
D.push(4);
D.push(5);         // 栈的现状为：    2  4
                                    1  3  5
                                    ﹈ ﹈ ﹈
D.popAtStack(0);   // 返回 2。栈的现状为：      4
                                          1  3  5
                                          ﹈ ﹈ ﹈
D.push(20);        // 栈的现状为：  20  4
                                   1  3  5
                                   ﹈ ﹈ ﹈
D.push(21);        // 栈的现状为：  20  4 21
                                   1  3  5
                                   ﹈ ﹈ ﹈
D.popAtStack(0);   // 返回 20。栈的现状为：       4 21
                                            1  3  5
                                            ﹈ ﹈ ﹈
D.popAtStack(2);   // 返回 21。栈的现状为：       4
                                            1  3  5
                                            ﹈ ﹈ ﹈
D.pop()            // 返回 5。栈的现状为：        4
                                            1  3
                                            ﹈ ﹈
D.pop()            // 返回 4。栈的现状为：    1  3
                                           ﹈ ﹈
D.pop()            // 返回 3。栈的现状为：    1
                                           ﹈
D.pop()            // 返回 1。现在没有栈。
D.pop()            // 返回 -1。仍然没有栈。
</pre>

#### 提示:
* <code>1 <= capacity <= 2 * 10<sup>4</sup></code>
* <code>1 <= val <= 2 * 10<sup>4</sup></code>
* <code>0 <= index <= 10<sup>5</sup></code>
* 最多会对 `push`，`pop`，和 `popAtStack` 进行 `200000` 次调用。

## 题解 (Rust)

### 1. 题解
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct DinnerPlates {
    capacity: usize,
    stacks: Vec<Vec<i32>>,
    non_full: BinaryHeap<Reverse<usize>>,
    non_empty: BinaryHeap<usize>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            stacks: vec![],
            non_full: BinaryHeap::new(),
            non_empty: BinaryHeap::new(),
        }
    }

    fn push(&mut self, val: i32) {
        while let Some(&Reverse(i)) = self.non_full.peek() {
            if self.stacks[i].len() == self.capacity {
                self.non_full.pop();
            } else {
                break;
            }
        }

        if self.non_full.is_empty() {
            self.non_full.push(Reverse(self.stacks.len()));
            self.stacks.push(Vec::with_capacity(self.capacity));
        }

        if let Some(&Reverse(i)) = self.non_full.peek() {
            if self.stacks[i].is_empty() {
                self.non_empty.push(i);
            }
            self.stacks[i].push(val);
        }
    }

    fn pop(&mut self) -> i32 {
        while let Some(&i) = self.non_empty.peek() {
            if self.stacks[i].is_empty() {
                self.non_empty.pop();
            } else {
                break;
            }
        }

        if self.non_empty.is_empty() {
            return -1;
        }

        self.pop_at_stack(*self.non_empty.peek().unwrap() as i32)
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let index = index as usize;

        if self.stacks.len() <= index || self.stacks[index].is_empty() {
            return -1;
        }

        if self.stacks[index].len() == self.capacity {
            self.non_full.push(Reverse(index));
        }
        self.stacks[index].pop().unwrap()
    }
}

/**
 * Your DinnerPlates object will be instantiated and called as such:
 * let obj = DinnerPlates::new(capacity);
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.pop_at_stack(index);
 */
```
