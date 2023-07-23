# 1663. Smallest String With A Given Numeric Value
The **numeric value** of a **lowercase character** is defined as its position `(1-indexed)` in the alphabet, so the numeric value of `a` is `1`, the numeric value of `b` is `2`, the numeric value of `c` is `3`, and so on.

The **numeric value** of a **string** consisting of lowercase characters is defined as the sum of its characters' numeric values. For example, the numeric value of the string `"abe"` is equal to `1 + 2 + 5 = 8`.

You are given two integers `n` and `k`. Return *the **lexicographically smallest string** with **length** equal to `n` and **numeric value** equal to `k`*.

Note that a string `x` is lexicographically smaller than string `y` if `x` comes before `y` in dictionary order, that is, either `x` is a prefix of `y`, or if `i` is the first position such that `x[i] != y[i]`, then `x[i]` comes before `y[i]` in alphabetic order.

#### Example 1:
<pre>
<strong>Input:</strong> n = 3, k = 27
<strong>Output:</strong> "aay"
<strong>Explanation:</strong> The numeric value of the string is 1 + 1 + 25 = 27, and it is the smallest string with such a value and length equal to 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 5, k = 73
<strong>Output:</strong> "aaszz"
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>5</sup></code>
* `n <= k <= 26 * n`

## Solutions (Ruby)

### 1. Greedy
```Ruby
# @param {Integer} n
# @param {Integer} k
# @return {String}
def get_smallest_string(n, k)
  k == 26 * n ? 'z' * n : 'a' * (n - (k - n) / 25 - 1) + (97 + (k - n) % 25).chr + 'z' * ((k - n) / 25)
end
```

## Solutions (Rust)

### 1. Greedy
```Rust
use std::iter;

impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        if k == 26 * n {
            "z".repeat(n as usize)
        } else {
            iter::repeat('a')
                .take((n - (k - n) / 25 - 1) as usize)
                .chain(iter::once((97 + (k - n) % 25) as u8 as char))
                .chain(iter::repeat('z').take(((k - n) / 25) as usize))
                .collect()
        }
    }
}
```
