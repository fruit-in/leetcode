# 927. Three Equal Parts
You are given an array `arr` which consists of only zeros and ones, divide the array into **three non-empty parts** such that all of these parts represent the same binary value.

If it is possible, return any `[i, j]` with `i + 1 < j`, such that:

* `arr[0], arr[1], ..., arr[i]` is the first part,
* `arr[i + 1], arr[i + 2], ..., arr[j - 1]` is the second part, and
* `arr[j], arr[j + 1], ..., arr[arr.length - 1]` is the third part.
* All three parts have equal binary values.

If it is not possible, return `[-1, -1]`.

Note that the entire part is used when considering what binary value it represents. For example, `[1,1,0]` represents `6` in decimal, not `3`. Also, leading zeros **are allowed**, so `[0,1,1]` and `[1,1]` represent the same value.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [1,0,1,0,1]
<strong>Output:</strong> [0,3]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,1,0,1,1]
<strong>Output:</strong> [-1,-1]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [1,1,0,0,1]
<strong>Output:</strong> [0,2]
</pre>

#### Constraints:
* <code>3 <= arr.length <= 3 * 10<sup>4</sup></code>
* `arr[i]` is `0` or `1`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let ones = arr.iter().filter(|&&x| x == 1).count() as i32;

        if ones % 3 != 0 {
            return vec![-1, -1];
        } else if ones == 0 {
            return vec![0, 2];
        }

        let mut i = 0;
        let mut j = 0;
        let mut k = arr.len() - 1;
        let mut count = 0;

        while arr[k] == 0 {
            k -= 1;
        }

        while count < ones / 3 {
            count += arr[i];
            i += 1;
        }
        for _ in k..arr.len() - 1 {
            if arr[i] == 1 {
                return vec![-1, -1];
            }

            i += 1;
        }
        i -= 1;

        j = i + 1;
        count = 0;
        while count < ones / 3 {
            count += arr[j];
            j += 1;
        }
        for _ in k..arr.len() - 1 {
            if arr[j] == 1 {
                return vec![-1, -1];
            }

            j += 1;
        }

        for k in 0..(i + 1).min(arr.len() - j).min(j - i - 1) {
            let (a, b, c) = (arr[i - k], arr[j - 1 - k], arr[arr.len() - 1 - k]);
            if a != b || b != c || a != c {
                return vec![-1, -1];
            }
        }

        vec![i as i32, j as i32]
    }
}
```
