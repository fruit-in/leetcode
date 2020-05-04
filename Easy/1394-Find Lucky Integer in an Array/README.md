# 1394. Find Lucky Integer in an Array
Given an array of integers ```arr```, a lucky integer is an integer which has a frequency in the array equal to its value.

Return *a lucky integer* in the array. If there are multiple lucky integers return the **largest** of them. If there is no lucky integer return **-1**.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [2,2,3,4]
<strong>Output:</strong> 2
<strong>Explanation:</strong> The only lucky number in the array is 2 because frequency[2] == 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,2,2,3,3,3]
<strong>Output:</strong> 3
<strong>Explanation:</strong> 1, 2 and 3 are all lucky numbers, return the largest of them.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [2,2,2,3,3]
<strong>Output:</strong> -1
<strong>Explanation:</strong> There are no lucky numbers in the array.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> arr = [5]
<strong>Output:</strong> -1
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> arr = [7,7,7,7,7,7,7]
<strong>Output:</strong> 7
</pre>

#### Constraints:
* ```1 <= arr.length <= 500```
* ```1 <= arr[i] <= 500```

## Solutions (Rust)

### 1. Count
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut cnt = HashMap::new();

        for n in arr {
            *cnt.entry(n).or_insert(0) += 1;
        }

        match cnt.iter().filter(|(k, v)| k == v).max_by_key(|(&k, &v)| k) {
            Some((&k, &v)) => k,
            None => -1,
        }
    }
}
```
