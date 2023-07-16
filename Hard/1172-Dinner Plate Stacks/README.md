# 1172. Dinner Plate Stacks
You have an infinite number of stacks arranged in a row and numbered (left to right) from `0`, each of the stacks has the same maximum capacity.

Implement the `DinnerPlates` class:

* `DinnerPlates(int capacity)` Initializes the object with the maximum capacity of the stacks `capacity`.
* `void push(int val)` Pushes the given integer `val` into the leftmost stack with a size less than `capacity`.
* `int pop()` Returns the value at the top of the rightmost non-empty stack and removes it from that stack, and returns `-1` if all the stacks are empty.
* `int popAtStack(int index)` Returns the value at the top of the stack with the given index `index` and removes it from that stack or returns `-1` if the stack with that given index is empty.

#### Example 1:
<pre>
<strong>Input:</strong>
["DinnerPlates", "push", "push", "push", "push", "push", "popAtStack", "push", "push", "popAtStack", "popAtStack", "pop", "pop", "pop", "pop", "pop"]
[[2], [1], [2], [3], [4], [5], [0], [20], [21], [0], [2], [], [], [], [], []]
<strong>Output:</strong>
[null, null, null, null, null, null, 2, null, null, 20, 21, 5, 4, 3, 1, -1]
<strong>Explanation:</strong>
DinnerPlates D = DinnerPlates(2);  // Initialize with capacity = 2
D.push(1);
D.push(2);
D.push(3);
D.push(4);
D.push(5);         // The stacks are now:  2  4
                                           1  3  5
                                           ﹈ ﹈ ﹈
D.popAtStack(0);   // Returns 2.  The stacks are now:     4
                                                       1  3  5
                                                       ﹈ ﹈ ﹈
D.push(20);        // The stacks are now: 20  4
                                           1  3  5
                                           ﹈ ﹈ ﹈
D.push(21);        // The stacks are now: 20  4 21
                                           1  3  5
                                           ﹈ ﹈ ﹈
D.popAtStack(0);   // Returns 20.  The stacks are now:     4 21
                                                        1  3  5
                                                        ﹈ ﹈ ﹈
D.popAtStack(2);   // Returns 21.  The stacks are now:     4
                                                        1  3  5
                                                        ﹈ ﹈ ﹈
D.pop()            // Returns 5.  The stacks are now:      4
                                                        1  3
                                                        ﹈ ﹈
D.pop()            // Returns 4.  The stacks are now:   1  3
                                                        ﹈ ﹈
D.pop()            // Returns 3.  The stacks are now:   1
                                                        ﹈
D.pop()            // Returns 1.  There are no stacks.
D.pop()            // Returns -1.  There are still no stacks.
</pre>

#### Constraints:
* <code>1 <= capacity <= 2 * 10<sup>4</sup></code>
* <code>1 <= val <= 2 * 10<sup>4</sup></code>
* <code>0 <= index <= 10<sup>5</sup></code>
* At most <code>2 * 10<sup>5</sup></code> calls will be made to `push`, `pop`, and `popAtStack`.

## Solutions (Rust)

### 1. Solution
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
