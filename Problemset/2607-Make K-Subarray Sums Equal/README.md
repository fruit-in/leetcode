# 2607. Make K-Subarray Sums Equal
You are given a **0-indexed** integer array `arr` and an integer `k`. The array `arr` is circular. In other words, the first element of the array is the next element of the last element, and the last element of the array is the previous element of the first element.

You can do the following operation any number of times:
* Pick any element from `arr` and increase or decrease it by `1`.

Return *the minimum number of operations such that the sum of each **subarray** of length* `k` *is equal*.

A **subarray** is a contiguous part of the array.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [1,4,1,3], k = 2
<strong>Output:</strong> 1
<strong>Explanation:</strong> we can do one operation on index 1 to make its value equal to 3.
The array after the operation is [1,3,1,3]
- Subarray starts at index 0 is [1, 3], and its sum is 4
- Subarray starts at index 1 is [3, 1], and its sum is 4
- Subarray starts at index 2 is [1, 3], and its sum is 4
- Subarray starts at index 3 is [3, 1], and its sum is 4
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [2,5,5,7], k = 3
<strong>Output:</strong> 5
<strong>Explanation:</strong> we can do three operations on index 0 to make its value equal to 5 and two operations on index 3 to make its value equal to 5.
The array after the operations is [5,5,5,5]
- Subarray starts at index 0 is [5, 5, 5], and its sum is 15
- Subarray starts at index 1 is [5, 5, 5], and its sum is 15
- Subarray starts at index 2 is [5, 5, 5], and its sum is 15
- Subarray starts at index 3 is [5, 5, 5], and its sum is 15
</pre>

#### Constraints:
* <code>1 <= k <= arr.length <= 10<sup>5</sup></code>
* <code>1 <= arr[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn make_sub_k_sum_equal(arr: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let n = arr.len();
        let mut visited = vec![false; n];
        let mut groups = HashMap::new();
        let mut ret = 0;

        for i in 0..n {
            if !visited[i] {
                let mut j = i;
                groups.insert(i, vec![]);

                while !visited[j] {
                    visited[j] = true;
                    groups.get_mut(&i).unwrap().push(arr[j]);
                    j = (j + k) % n;
                }
            }
        }

        for mut group in groups.into_values() {
            let i = group.len() / 2;
            group.sort_unstable();

            for j in 0..group.len() {
                ret += (group[j] - group[i]).abs() as i64;
            }
        }

        ret
    }
}
```
