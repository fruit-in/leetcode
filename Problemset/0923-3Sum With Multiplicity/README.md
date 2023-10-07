# 923. 3Sum With Multiplicity
Given an integer array `arr`, and an integer `target`, return the number of tuples `i, j, k` such that `i < j < k` and `arr[i] + arr[j] + arr[k] == target`.

As the answer can be very large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [1,1,2,2,3,3,4,4,5,5], target = 8
<strong>Output:</strong> 20
<strong>Explanation:</strong>
Enumerating by the values (arr[i], arr[j], arr[k]):
(1, 2, 5) occurs 8 times;
(1, 3, 4) occurs 8 times;
(2, 2, 4) occurs 2 times;
(2, 3, 3) occurs 2 times.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,1,2,2,2,2], target = 5
<strong>Output:</strong> 12
<strong>Explanation:</strong>
arr[i] = 1, arr[j] = arr[k] = 2 occurs 12 times:
We choose one 1 from [1,1] in 2 ways,
and two 2s from [2,2,2,2] in 6 ways.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [2,1,3], target = 6
<strong>Output:</strong> 1
<strong>Explanation:</strong> (1, 2, 3) occured one time in the array so we return 1.
</pre>

#### Constraints:
* `3 <= arr.length <= 3000`
* `0 <= arr[i] <= 100`
* `0 <= target <= 300`

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        let mut hashmap = HashMap::new();
        let mut ret = 0;

        for j in 0..arr.len() {
            ret = (ret + hashmap.get(&(target - arr[j])).unwrap_or(&0)) % 1_000_000_007;

            for i in 0..j {
                hashmap
                    .entry(arr[i] + arr[j])
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }

        ret
    }
}
```
