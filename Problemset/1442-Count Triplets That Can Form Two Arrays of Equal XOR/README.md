# 1442. Count Triplets That Can Form Two Arrays of Equal XOR
Given an array of integers `arr`.

We want to select three indices `i`, `j` and `k` where `(0 <= i < j <= k < arr.length)`.

Let's define `a` and `b` as follows:
* `a = arr[i] ^ arr[i + 1] ^ ... ^ arr[j - 1]`
* `b = arr[j] ^ arr[j + 1] ^ ... ^ arr[k]`

Note that **^** denotes the **bitwise-xor** operation.

Return *the number of triplets* (`i`, `j` and `k`) Where `a == b`.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [2,3,1,6,7]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The triplets are (0,1,2), (0,2,2), (2,3,4) and (2,4,4)
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,1,1,1,1]
<strong>Output:</strong> 10
</pre>

#### Constraints:
* `1 <= arr.length <= 300`
* <code>1 <= arr[i] <= 10<sup>8</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut prefix_xor = vec![0; arr.len() + 1];
        let mut prefix_xor_indices = HashMap::from([(0, vec![0])]);
        let mut ret = 0;

        for k in 0..arr.len() {
            prefix_xor[k + 1] = prefix_xor[k] ^ arr[k];
            for &i in prefix_xor_indices
                .get(&prefix_xor[k + 1])
                .unwrap_or(&vec![])
            {
                ret += k - i;
            }
            prefix_xor_indices
                .entry(prefix_xor[k + 1])
                .or_insert(vec![])
                .push(k + 1);
        }

        ret as i32
    }
}
```
