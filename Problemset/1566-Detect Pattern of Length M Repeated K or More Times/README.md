# 1566. Detect Pattern of Length M Repeated K or More Times
Given an array of positive integers `arr`,  find a pattern of length `m` that is repeated `k` or more times.

A **pattern** is a subarray (consecutive sub-sequence) that consists of one or more values, repeated multiple times **consecutively** without overlapping. A pattern is defined by its length and the number of repetitions.

Return `true` *if there exists a pattern of length* `m` *that is repeated* `k` *or more times, otherwise return* `false`.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [1,2,4,4,4,4], m = 1, k = 3
<strong>Output:</strong> true
<strong>Explanation:</strong> The pattern <b>(4)</b> of length 1 is repeated 4 consecutive times. Notice that pattern can be repeated k or more times but not less.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,2,1,2,1,1,1,3], m = 2, k = 2
<strong>Output:</strong> true
<strong>Explanation:</strong> The pattern <b>(1,2)</b> of length 2 is repeated 2 consecutive times. Another valid pattern <b>(2,1)</b> is also repeated 2 times.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [1,2,1,2,1,3], m = 2, k = 3
<strong>Output:</strong> false
<strong>Explanation:</strong> The pattern (1,2) is of length 2 but is repeated only 2 times. There is no pattern of length 2 that is repeated 3 or more times.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> arr = [1,2,3,1,2], m = 2, k = 2
<strong>Output:</strong> false
<strong>Explanation:</strong> Notice that the pattern (1,2) exists twice but not consecutively, so it doesn't count.
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> arr = [2,2,2,2], m = 2, k = 3
<strong>Output:</strong> false
<strong>Explanation:</strong> The only pattern of length 2 is (2,2) however it's repeated only twice. Notice that we do not count overlapping repetitions.
</pre>

#### Constraints:
* `2 <= arr.length <= 100`
* `1 <= arr[i] <= 100`
* `1 <= m <= 100`
* `2 <= k <= 100`

## Solutions (Rust)

### 1. Brute Force
```Rust
impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        let m = m as usize;
        let k = k as usize;

        if arr.len() < m * k {
            return false;
        }

        for i in 0..=(arr.len() - m * k) {
            if (i..(i + m * (k - 1))).all(|j| arr[j] == arr[j + m]) {
                return true;
            }
        }

        false
    }
}
```
