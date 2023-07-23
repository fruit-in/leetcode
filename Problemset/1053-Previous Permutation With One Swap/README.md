# 1053. Previous Permutation With One Swap
Given an array of positive integers `arr` (not necessarily distinct), return *the lexicographically largest permutation that is smaller than* `arr`, that can be **made with exactly one swap** (A *swap* exchanges the positions of two numbers `arr[i]` and `arr[j]`). If it cannot be done, then return the same array.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [3,2,1]
<strong>Output:</strong> [3,1,2]
<strong>Explanation:</strong> Swapping 2 and 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,1,5]
<strong>Output:</strong> [1,1,5]
<strong>Explanation:</strong> This is already the smallest permutation.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [1,9,4,6,7]
<strong>Output:</strong> [1,7,4,6,9]
<strong>Explanation:</strong> Swapping 9 and 7.
</pre>

#### Constraints:
* <code>1 <= arr.length <= 10<sup>4</sup></code>
* <code>1 <= arr[i] <= 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn prev_perm_opt1(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;

        for i in (0..arr.len() - 1).rev() {
            if arr[i] > arr[i + 1] {
                let mut j = i + 1;

                for k in i + 2..arr.len() {
                    if arr[j] < arr[k] && arr[i] > arr[k] {
                        j = k;
                    }
                }

                arr.swap(i, j);

                break;
            }
        }

        arr
    }
}
```
