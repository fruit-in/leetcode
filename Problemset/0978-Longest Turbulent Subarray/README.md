# 978. Longest Turbulent Subarray
Given an integer array `arr`, return the length of a maximum size turbulent subarray of `arr`.

A subarray is turbulent if the comparison sign flips between each adjacent pair of elements in the subarray.

More formally, a subarray `[arr[i], arr[i + 1], ..., arr[j]]` of `arr` is said to be turbulent if and only if:
* For `i <= k < j`:
    * `arr[k] > arr[k + 1]` when `k` is odd, and
    * `arr[k] < arr[k + 1]` when `k` is even.
* Or, for `i <= k < j`:
    * `arr[k] > arr[k + 1]` when `k` is even, and
    * `arr[k] < arr[k + 1]` when `k` is odd.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [9,4,2,10,7,8,8,1,9]
<strong>Output:</strong> 5
<strong>Explanation:</strong> arr[1] > arr[2] < arr[3] > arr[4] < arr[5]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [4,8,12,16]
<strong>Output:</strong> 2
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [100]
<strong>Output:</strong> 1
</pre>

#### Constraints:
* <code>1 <= arr.length <= 4 * 10<sup>4</sup></code>
* <code>0 <= arr[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let mut count0 = 1;
        let mut count1 = 1;
        let mut ret = 1;

        for k in 0..arr.len() - 1 {
            if (k % 2 == 1 && arr[k] > arr[k + 1]) || (k % 2 == 0 && arr[k] < arr[k + 1]) {
                count0 += 1;
            } else {
                ret = ret.max(count0);
                count0 = 1;
            }
            if (k % 2 == 0 && arr[k] > arr[k + 1]) || (k % 2 == 1 && arr[k] < arr[k + 1]) {
                count1 += 1;
            } else {
                ret = ret.max(count1);
                count1 = 1;
            }
        }

        ret.max(count0).max(count1)
    }
}
```
