# 2336. Smallest Number in Infinite Set
You have a set which contains all positive integers `[1, 2, 3, 4, 5, ...]`.

Implement the `SmallestInfiniteSet` class:

* `SmallestInfiniteSet()` Initializes the **SmallestInfiniteSet** object to contain **all** positive integers.
* `int popSmallest()` **Removes** and returns the smallest integer contained in the infinite set.
* `void addBack(int num)` **Adds** a positive integer `num` back into the infinite set, if it is **not** already in the infinite set.

#### Example 1:
<pre>
<strong>Input:</strong>
["SmallestInfiniteSet", "addBack", "popSmallest", "popSmallest", "popSmallest", "addBack", "popSmallest", "popSmallest", "popSmallest"]
[[], [2], [], [], [], [1], [], [], []]
<strong>Output:</strong>
[null, null, 1, 2, 3, null, 1, 4, 5]
<strong>Explanation:</strong>
SmallestInfiniteSet smallestInfiniteSet = new SmallestInfiniteSet();
smallestInfiniteSet.addBack(2);    // 2 is already in the set, so no change is made.
smallestInfiniteSet.popSmallest(); // return 1, since 1 is the smallest number, and remove it from the set.
smallestInfiniteSet.popSmallest(); // return 2, and remove it from the set.
smallestInfiniteSet.popSmallest(); // return 3, and remove it from the set.
smallestInfiniteSet.addBack(1);    // 1 is added back to the set.
smallestInfiniteSet.popSmallest(); // return 1, since 1 was added back to the set and
                                   // is the smallest number, and remove it from the set.
smallestInfiniteSet.popSmallest(); // return 4, and remove it from the set.
smallestInfiniteSet.popSmallest(); // return 5, and remove it from the set.
</pre>

#### Constraints:
* `1 <= num <= 1000`
* At most `1000` calls will be made **in total** to `popSmallest` and `addBack`.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;
use std::collections::HashSet;

struct SmallestInfiniteSet {
    min_infinite: i32,
    heap: BinaryHeap<i32>,
    set: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {
    fn new() -> Self {
        Self {
            min_infinite: 1,
            heap: BinaryHeap::new(),
            set: HashSet::new(),
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        match self.heap.pop() {
            Some(x) => self.set.take(&(-x)).unwrap(),
            None => {
                self.min_infinite += 1;
                self.min_infinite - 1
            }
        }
    }

    fn add_back(&mut self, num: i32) {
        if num < self.min_infinite && self.set.insert(num) {
            self.heap.push(-num);
        }
    }
}

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */
```
