# 2526. Find Consecutive Integers from a Data Stream
For a stream of integers, implement a data structure that checks if the last `k` integers parsed in the stream are **equal** to `value`.

Implement the **DataStream** class:

* `DataStream(int value, int k)` Initializes the object with an empty integer stream and the two integers `value` and `k`.
* `boolean consec(int num)` Adds `num` to the stream of integers. Returns `true` if the last `k` integers are equal to `value`, and `false` otherwise. If there are less than `k` integers, the condition does not hold true, so returns `false`.

#### Example 1:
<pre>
<strong>Input:</strong>
["DataStream", "consec", "consec", "consec", "consec"]
[[4, 3], [4], [4], [4], [3]]
<strong>Output:</strong>
[null, false, false, true, false]
<strong>Explanation:</strong>
DataStream dataStream = new DataStream(4, 3); //value = 4, k = 3
dataStream.consec(4); // Only 1 integer is parsed, so returns False.
dataStream.consec(4); // Only 2 integers are parsed.
                      // Since 2 is less than k, returns False.
dataStream.consec(4); // The 3 integers parsed are all equal to value, so returns True.
dataStream.consec(3); // The last k integers parsed in the stream are [4,4,3].
                      // Since 3 is not equal to value, it returns False.
</pre>

#### Constraints:
* <code>1 <= value, num <= 10<sup>9</sup></code>
* <code>1 <= k <= 10<sup>5</sup></code>
* At most <code>10<sup>5</sup></code> calls will be made to `consec`.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::VecDeque;

struct DataStream {
    deque: VecDeque<i32>,
    value: i32,
    k: usize,
    count: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl DataStream {
    fn new(value: i32, k: i32) -> Self {
        Self {
            deque: VecDeque::with_capacity(k as usize),
            value,
            k: k as usize,
            count: 0,
        }
    }

    fn consec(&mut self, num: i32) -> bool {
        if self.deque.len() == self.k && self.deque.pop_front().unwrap() == self.value {
            self.count -= 1;
        }

        self.deque.push_back(num);

        if num == self.value {
            self.count += 1;
        }

        self.count == self.k
    }
}

/**
 * Your DataStream object will be instantiated and called as such:
 * let obj = DataStream::new(value, k);
 * let ret_1: bool = obj.consec(num);
 */
```
