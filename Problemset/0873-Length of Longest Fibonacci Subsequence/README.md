# 873. Length of Longest Fibonacci Subsequence
A sequence `x1, x2, ..., xn` is *Fibonacci-like* if:
* `n >= 3`
* `xi + xi+1 == xi+2` for all `i + 2 <= n`

Given a **strictly increasing** array `arr` of positive integers forming a sequence, return *the **length** of the longest Fibonacci-like subsequence of* `arr`. If one does not exist, return `0`.

A **subsequence** is derived from another sequence `arr` by deleting any number of elements (including none) from `arr`, without changing the order of the remaining elements. For example, `[3, 5, 8]` is a subsequence of `[3, 4, 5, 6, 7, 8]`.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [1,2,3,4,5,6,7,8]
<strong>Output:</strong> 5
<strong>Explanation:</strong> The longest subsequence that is fibonacci-like: [1,2,3,5,8].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,3,7,11,12,14,18]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The longest subsequence that is fibonacci-like: [1,11,12], [3,11,14] or [7,11,18].
</pre>

#### Constraints:
* `3 <= arr.length <= 1000`
* <code>1 <= arr[i] < arr[i + 1] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let mut lengths = HashMap::new();
        let last = *arr.last().unwrap();
        let mut ret = 0;

        for i in 1..arr.len() {
            for j in 0..i {
                if arr[i] - arr[j] < arr[j] {
                    let x = *lengths.get(&(arr[i] - arr[j], arr[j])).unwrap_or(&1);

                    if x > 1 {
                        lengths.insert((arr[j], arr[i]), x + 1);
                        ret = ret.max(x + 1);
                    } else if arr[i] + arr[j] <= last {
                        lengths.insert((arr[j], arr[i]), 2);
                    }
                } else if arr[i] + arr[j] <= last {
                    lengths.insert((arr[j], arr[i]), 2);
                }
            }
        }

        ret
    }
}
```
