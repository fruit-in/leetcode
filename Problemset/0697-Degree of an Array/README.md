# 697. Degree of an Array
Given a non-empty array of non-negative integers ```nums```, the **degree** of this array is defined as the maximum frequency of any one of its elements.

Your task is to find the smallest possible length of a (contiguous) subarray of ```nums```, that has the same degree as ```nums```.

#### Example 1:
<pre>
<strong>Input:</strong> [1, 2, 2, 3, 1]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
The input array has a degree of 2 because both elements 1 and 2 appear twice.
Of the subarrays that have the same degree:
[1, 2, 2, 3, 1], [1, 2, 2, 3], [2, 2, 3, 1], [1, 2, 2], [2, 2, 3], [2, 2]
The shortest length is 2. So return 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [1,2,2,3,1,4,2]
<strong>Output:</strong> 6
</pre>

#### Note:
* ```nums.length``` will be between 1 and 50,000.
* ```nums[i]``` will be an integer between 0 and 49,999.

## Solutions (Rust)

### 1. HashMap
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut cnt_left_len = HashMap::new();
        let mut degree = 0;

        for i in 0..nums.len() {
            let cll = cnt_left_len.entry(nums[i]).or_insert([0, i, 1]);
            cll[0] += 1;
            cll[2] = i - cll[1] + 1;
            degree = degree.max(cll[0])
        }

        cnt_left_len.values()
                    .filter(|arr| arr[0] == degree)
                    .map(|arr| arr[2])
                    .min()
                    .unwrap() as i32
    }
}
```
