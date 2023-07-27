# 2349. 设计数字容器系统
设计一个数字容器系统，可以实现以下功能：

* 在系统中给定下标处 **插入** 或者 **替换** 一个数字。
* **返回** 系统中给定数字的最小下标。

请你实现一个 `NumberContainers` 类：

* `NumberContainers()` 初始化数字容器系统。
* `void change(int index, int number)` 在下标 `index` 处填入 `number` 。如果该下标 `index` 处已经有数字了，那么用 `number` 替换该数字。
* `int find(int number)` 返回给定数字 `number` 在系统中的最小下标。如果系统中没有 `number` ，那么返回 `-1` 。

#### 示例:
<pre>
<strong>输入:</strong>
["NumberContainers", "find", "change", "change", "change", "change", "find", "change", "find"]
[[], [10], [2, 10], [1, 10], [3, 10], [5, 10], [10], [1, 20], [10]]
<strong>输出:</strong>
[null, -1, null, null, null, null, 1, null, 2]
<strong>解释:</strong>
NumberContainers nc = new NumberContainers();
nc.find(10); // 没有数字 10 ，所以返回 -1 。
nc.change(2, 10); // 容器中下标为 2 处填入数字 10 。
nc.change(1, 10); // 容器中下标为 1 处填入数字 10 。
nc.change(3, 10); // 容器中下标为 3 处填入数字 10 。
nc.change(5, 10); // 容器中下标为 5 处填入数字 10 。
nc.find(10); // 数字 10 所在的下标为 1 ，2 ，3 和 5 。因为最小下标为 1 ，所以返回 1 。
nc.change(1, 20); // 容器中下标为 1 处填入数字 20 。注意，下标 1 处之前为 10 ，现在被替换为 20 。
nc.find(10); // 数字 10 所在下标为 2 ，3 和 5 。最小下标为 2 ，所以返回 2 。
</pre>

#### 提示:
* <code>1 <= index, number <= 10<sup>9</sup></code>
* 调用 `change` 和 `find` 的 **总次数** 不超过 <code>10<sup>5</sup></code> 次。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;
use std::collections::HashMap;

struct NumberContainers {
    nums: HashMap<i32, i32>,
    indices: HashMap<i32, BinaryHeap<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumberContainers {
    fn new() -> Self {
        Self {
            nums: HashMap::new(),
            indices: HashMap::new(),
        }
    }

    fn change(&mut self, index: i32, number: i32) {
        self.nums.insert(index, number);
        self.indices
            .entry(number)
            .and_modify(|h| h.push(-index))
            .or_insert(BinaryHeap::from([-index]));
    }

    fn find(&mut self, number: i32) -> i32 {
        if !self.indices.contains_key(&number) {
            return -1;
        }

        while let Some(&i) = self.indices[&number].peek() {
            if self.nums[&-i] != number {
                self.indices.get_mut(&number).unwrap().pop();
            } else {
                return -i;
            }
        }

        -1
    }
}

/**
 * Your NumberContainers object will be instantiated and called as such:
 * let obj = NumberContainers::new();
 * obj.change(index, number);
 * let ret_2: i32 = obj.find(number);
 */
```
