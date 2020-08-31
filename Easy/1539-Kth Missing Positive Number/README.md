# 1539. Kth Missing Positive Number
Given an array `arr` of positive integers sorted in a **strictly increasing order**, and an integer `k`.

*Find the* `k`<sup>`th`</sup> *positive integer that is missing from this array.*

#### Example 1:
<pre>
<strong>Input:</strong> arr = [2,3,4,7,11], k = 5
<strong>Output:</strong> 9
<strong>Explanation:</strong> The missing positive integers are [1,5,6,8,9,10,12,13,...]. The 5<sup>th</sup> missing positive integer is 9.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,2,3,4], k = 2
<strong>Output:</strong> 6
<strong>Explanation:</strong> The missing positive integers are [5,6,7,...]. The 2<sup>nd</sup> missing positive integer is 6.
</pre>

#### Constraints:
* `1 <= arr.length <= 1000`
* `1 <= arr[i] <= 1000`
* `1 <= k <= 1000`
* `arr[i] < arr[j]` for `1 <= i < j <= arr.length`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut k = k;
        let mut i = 0;

        for n in 1..2001 {
            if i >= arr.len() || n < arr[i] {
                k -= 1;
            } else {
                i += 1;
            }
            if k == 0 {
                return n;
            }
        }

        0
    }
}
```
