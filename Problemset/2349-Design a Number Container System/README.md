# 2349. Design a Number Container System
Design a number container system that can do the following:

* **Insert** or **Replace** a number at the given index in the system.
* **Return** the smallest index for the given number in the system.

Implement the `NumberContainers` class:

* `NumberContainers()` Initializes the number container system.
* `void change(int index, int number)` Fills the container at index with the `number`. If there is already a number at that `index`, replace it.
* `int find(int number)` Returns the smallest index for the given `number`, or `-1` if there is no index that is filled by `number` in the system.

#### Example 1:
<pre>
<strong>Input:</strong>
["NumberContainers", "find", "change", "change", "change", "change", "find", "change", "find"]
[[], [10], [2, 10], [1, 10], [3, 10], [5, 10], [10], [1, 20], [10]]
<strong>Output:</strong>
[null, -1, null, null, null, null, 1, null, 2]
<strong>Explanation:</strong>
NumberContainers nc = new NumberContainers();
nc.find(10); // There is no index that is filled with number 10. Therefore, we return -1.
nc.change(2, 10); // Your container at index 2 will be filled with number 10.
nc.change(1, 10); // Your container at index 1 will be filled with number 10.
nc.change(3, 10); // Your container at index 3 will be filled with number 10.
nc.change(5, 10); // Your container at index 5 will be filled with number 10.
nc.find(10); // Number 10 is at the indices 1, 2, 3, and 5. Since the smallest index that is filled with 10 is 1, we return 1.
nc.change(1, 20); // Your container at index 1 will be filled with number 20. Note that index 1 was filled with 10 and then replaced with 20.
nc.find(10); // Number 10 is at the indices 2, 3, and 5. The smallest index that is filled with 10 is 2. Therefore, we return 2.
</pre>

#### Constraints:
* <code>1 <= index, number <= 10<sup>9</sup></code>
* At most <code>10<sup>5</sup></code> calls will be made **in total** to `change` and `find`.

## Solutions (Rust)

### 1. Solution
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
