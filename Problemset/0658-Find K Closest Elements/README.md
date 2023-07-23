# 658. Find K Closest Elements
Given a **sorted** integer array `arr`, two integers `k` and `x`, return the `k` closest integers to `x` in the array. The result should also be sorted in ascending order.

An integer `a` is closer to `x` than an integer `b` if:
* `|a - x| < |b - x|`, or
* `|a - x| == |b - x|` and `a < b`

#### Example 1:
<pre>
<strong>Input:</strong> arr = [1,2,3,4,5], k = 4, x = 3
<strong>Output:</strong> [1,2,3,4]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,2,3,4,5], k = 4, x = -1
<strong>Output:</strong> [1,2,3,4]
</pre>

#### Constraints:
* `1 <= k <= arr.length`
* <code>1 <= arr.length <= 10<sup>4</sup></code>
* `arr` is sorted in **ascending** order.
* <code>-10<sup>4</sup> <= arr[i], x <= 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Binary Search and Two Pointers
```Rust
impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let mut l = match arr.binary_search(&x) {
            Err(i) if i == arr.len() => return arr.split_at(i - k as usize).1.to_vec(),
            Err(i) if i == 0 => return arr[0..k as usize].to_vec(),
            Err(i) if (arr[i - 1] - x).abs() <= arr[i] - x => i - 1,
            Ok(i) | Err(i) => i,
        };
        let mut r = l + 1;

        while ((r - l) as i32) < k {
            if r == arr.len() || (l > 0 && (arr[l - 1] - x).abs() <= arr[r] - x) {
                l -= 1;
            } else {
                r += 1;
            }
        }

        arr[l..r].to_vec()
    }
}
```

### 2. Sort
```Rust
impl Solution {
    pub fn find_closest_elements(mut arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        arr.sort_unstable_by_key(|&y| ((y - x).abs(), y));
        arr.truncate(k as usize);
        arr.sort_unstable();

        arr
    }
}
```
