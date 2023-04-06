# 2336. 无限集中的最小数字
现有一个包含所有正整数的集合 `[1, 2, 3, 4, 5, ...]` 。

实现 `SmallestInfiniteSet` 类：

* `SmallestInfiniteSet()` 初始化 **SmallestInfiniteSet** 对象以包含 **所有** 正整数。
* `int popSmallest()` **移除** 并返回该无限集中的最小整数。
* `void addBack(int num)` 如果正整数 `num` **不** 存在于无限集中，则将一个 `num` **添加** 到该无限集中。

#### 示例:
<pre>
<strong>输入:</strong>
["SmallestInfiniteSet", "addBack", "popSmallest", "popSmallest", "popSmallest", "addBack", "popSmallest", "popSmallest", "popSmallest"]
[[], [2], [], [], [], [1], [], [], []]
<strong>输出:</strong>
[null, null, 1, 2, 3, null, 1, 4, 5]
<strong>解释:</strong>
SmallestInfiniteSet smallestInfiniteSet = new SmallestInfiniteSet();
smallestInfiniteSet.addBack(2);    // 2 已经在集合中，所以不做任何变更。
smallestInfiniteSet.popSmallest(); // 返回 1 ，因为 1 是最小的整数，并将其从集合中移除。
smallestInfiniteSet.popSmallest(); // 返回 2 ，并将其从集合中移除。
smallestInfiniteSet.popSmallest(); // 返回 3 ，并将其从集合中移除。
smallestInfiniteSet.addBack(1);    // 将 1 添加到该集合中。
smallestInfiniteSet.popSmallest(); // 返回 1 ，因为 1 在上一步中被添加到集合中，
                                   // 且 1 是最小的整数，并将其从集合中移除。
smallestInfiniteSet.popSmallest(); // 返回 4 ，并将其从集合中移除。
smallestInfiniteSet.popSmallest(); // 返回 5 ，并将其从集合中移除。
</pre>

#### 提示:
* `1 <= num <= 1000`
* 最多调用 `popSmallest` 和 `addBack` 方法 共计 `1000` 次

## 题解 (Rust)

### 1. 题解
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
