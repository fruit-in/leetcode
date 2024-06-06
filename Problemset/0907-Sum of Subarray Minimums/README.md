# 907. Sum of Subarray Minimums
Given an array of integers arr, find the sum of `min(b)`, where `b` ranges over every (contiguous) subarray of `arr`. Since the answer may be large, return the answer **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [3,1,2,4]
<strong>Output:</strong> 17
<strong>Explanation:</strong>
Subarrays are [3], [1], [2], [4], [3,1], [1,2], [2,4], [3,1,2], [1,2,4], [3,1,2,4].
Minimums are 3, 1, 2, 4, 1, 1, 2, 1, 1, 1.
Sum is 17.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [11,81,94,43,3]
<strong>Output:</strong> 444
</pre>

#### Constraints:
* <code>1 <= arr.length <= 3 * 10<sup>4</sup></code>
* <code>1 <= arr[i] <= 3 * 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut less = vec![(0, 0); arr.len()];
        let mut ret = 0;

        for i in 0..arr.len() {
            while stack.last().unwrap_or(&(0, 0)).1 >= arr[i] {
                stack.pop();
            }

            less[i].0 = i as i64 - stack.last().unwrap_or(&(-1, 0)).0;
            stack.push((i as i64, arr[i]));
        }

        stack.clear();

        for i in (0..arr.len()).rev() {
            while stack.last().unwrap_or(&(0, 0)).1 > arr[i] {
                stack.pop();
            }

            less[i].1 = stack.last().unwrap_or(&(arr.len() as i64, 0)).0 - i as i64;
            ret = (ret + arr[i] as i64 * less[i].0 * less[i].1) % 1_000_000_007;
            stack.push((i as i64, arr[i]));
        }

        ret as i32
    }
}
```
