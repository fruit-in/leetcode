# 845. Longest Mountain in Array
You may recall that an array `arr` is a **mountain array** if and only if:
* `arr.length >= 3`
* There exists some index `i` (**0-indexed**) with `0 < i < arr.length - 1` such that:
    * `arr[0] < arr[1] < ... < arr[i - 1] < arr[i]`
    * `arr[i] > arr[i + 1] > ... > arr[arr.length - 1]`

Given an integer array `arr`, return *the length of the longest subarray, which is a mountain*. Return `0` if there is no mountain subarray.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [2,1,4,7,3,2,5]
<strong>Output:</strong> 5
<strong>Explanation:</strong> The largest mountain is [1,4,7,3,2] which has length 5.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [2,2,2]
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no mountain.
</pre>

#### Constraints:
* <code>1 <= arr.length <= 10<sup>4</sup></code>
* <code>0 <= arr[i] <= 10<sup>4</sup></code>

#### Follow up:
* Can you solve it using only one pass?
* Can you solve it in `O(1)` space?

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn longest_mountain(arr: Vec<i32>) -> i32 {
        let mut upcount = 0;
        let mut downcount = 0;
        let mut ret = 0;

        for i in 1..arr.len() {
            if arr[i] == arr[i - 1] {
                upcount = 0;
                downcount = 0;
            } else if arr[i] > arr[i - 1] && i > 1 && arr[i - 1] < arr[i - 2] {
                upcount = 1;
                downcount = 0;
            } else if arr[i] > arr[i - 1] {
                upcount += 1;
            } else {
                downcount += 1;
            }

            if upcount > 0 && downcount > 0 {
                ret = ret.max(upcount + downcount + 1);
            }
        }

        ret
    }
}
```
