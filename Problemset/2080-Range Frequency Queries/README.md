# 2080. Range Frequency Queries
Design a data structure to find the **frequency** of a given value in a given subarray.

The **frequency** of a value in a subarray is the number of occurrences of that value in the subarray.

Implement the `RangeFreqQuery` class:

* `RangeFreqQuery(int[] arr)` Constructs an instance of the class with the given **0-indexed** integer array `arr`.
* `int query(int left, int right, int value)` Returns the **frequency** of `value` in the subarray `arr[left...right]`.

A **subarray** is a contiguous sequence of elements within an array. `arr[left...right]` denotes the subarray that contains the elements of `nums` between indices `left` and `right` (**inclusive**).

#### Example 1:
<pre>
<strong>Input:</strong>
["RangeFreqQuery", "query", "query"]
[[[12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]], [1, 2, 4], [0, 11, 33]]
<strong>Output:</strong>
[null, 1, 2]
<strong>Explanation:</strong>
RangeFreqQuery rangeFreqQuery = new RangeFreqQuery([12, 33, 4, 56, 22, 2, 34, 33, 22, 12, 34, 56]);
rangeFreqQuery.query(1, 2, 4); // return 1. The value 4 occurs 1 time in the subarray [33, 4]
rangeFreqQuery.query(0, 11, 33); // return 2. The value 33 occurs 2 times in the whole array.
</pre>

#### Constraints:
* <code>1 <= arr.length <= 10<sup>5</sup></code>
* <code>1 <= arr[i], value <= 10<sup>4</sup></code>
* `0 <= left <= right < arr.length`
* At most <code>10<sup>5</sup></code> calls will be made to `query`

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

struct RangeFreqQuery {
    indices: HashMap<i32, Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeFreqQuery {
    fn new(arr: Vec<i32>) -> Self {
        let mut indices = HashMap::new();

        for i in 0..arr.len() {
            indices
                .entry(arr[i])
                .and_modify(|v: &mut Vec<i32>| v.push(i as i32))
                .or_insert(vec![i as i32]);
        }

        Self { indices }
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        if !self.indices.contains_key(&value) {
            return 0;
        }

        let indices = self.indices.get(&value).unwrap();
        let left = match indices.binary_search(&left) {
            Ok(i) | Err(i) => i as i32,
        };
        let right = match indices.binary_search(&right) {
            Ok(i) => i as i32,
            Err(i) => i as i32 - 1,
        };

        right - left + 1
    }
}

/**
 * Your RangeFreqQuery object will be instantiated and called as such:
 * let obj = RangeFreqQuery::new(arr);
 * let ret_1: i32 = obj.query(left, right, value);
 */
```
