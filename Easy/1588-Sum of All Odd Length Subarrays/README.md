# 1588. Sum of All Odd Length Subarrays
Given an array of positive integers `arr`, calculate the sum of all possible odd-length subarrays.

A subarray is a contiguous subsequence of the array.

Return *the sum of all odd-length subarrays of* `arr`.

#### Example 1:
<pre>
<b>Input:</b> arr = [1,4,2,5,3]
<b>Output:</b> 58
<b>Explanation:</b> The odd-length subarrays of arr and their sums are:
[1] = 1
[4] = 4
[2] = 2
[5] = 5
[3] = 3
[1,4,2] = 7
[4,2,5] = 11
[2,5,3] = 10
[1,4,2,5,3] = 15
If we add all these together we get 1 + 4 + 2 + 5 + 3 + 7 + 11 + 10 + 15 = 58
</pre>

#### Example 2:
<pre>
<b>Input:</b> arr = [1,2]
<b>Output:</b> 3
<b>Explanation:</b> There are only 2 subarrays of odd length, [1] and [2]. Their sum is 3.
</pre>

#### Example 3:
<pre>
<b>Input:</b> arr = [10,11,12]
<b>Output:</b> 66
</pre>

#### Constraints:
* `1 <= arr.length <= 100`
* `1 <= arr[i] <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut d = (arr.len() as i32 + 1) / 2;
        let mut prev = d;
        let mut ret = 0;

        for i in 0..(arr.len() / 2) {
            let j = arr.len() - 1 - i;

            ret += (arr[i] + arr[j]) * prev;
            d -= match arr.len() % 2 {
                0 => 1,
                _ => 2 * (1 - i as i32 % 2),
            };
            prev += d;
        }

        if arr.len() % 2 == 1 {
            ret += arr[arr.len() / 2] * prev;
        }

        ret
    }
}
```
