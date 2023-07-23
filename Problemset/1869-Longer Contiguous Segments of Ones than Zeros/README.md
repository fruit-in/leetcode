# 1869. Longer Contiguous Segments of Ones than Zeros
Given a binary string `s`, return `true` *if the **longest** contiguous segment of* `1`*'s is **strictly longer** than the **longest** contiguous segment of* `0`*'s in* `s`, or return `false` *otherwise*.
* For example, in `s = "110100010"` the longest continuous segment of `1`s has length `2`, and the longest continuous segment of `0`s has length `3`.

Note that if there are no `0`'s, then the longest continuous segment of `0`'s is considered to have a length `0`. The same applies if there is no `1`'s.

#### Example 1:
<pre>
<strong>Input:</strong> s = "1101"
<strong>Output:</strong> true
<strong>Explanation:</strong>
The longest contiguous segment of 1s has length 2: "1101"
The longest contiguous segment of 0s has length 1: "1101"
The segment of 1s is longer, so return true.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "111000"
<strong>Output:</strong> false
<strong>Explanation:</strong>
The longest contiguous segment of 1s has length 3: "111000"
The longest contiguous segment of 0s has length 3: "111000"
The segment of 1s is not longer, so return false.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "110100010"
<strong>Output:</strong> false
<strong>Explanation:</strong>
The longest contiguous segment of 1s has length 2: "110100010"
The longest contiguous segment of 0s has length 3: "110100010"
The segment of 1s is not longer, so return false.
</pre>

#### Constraints:
* `1 <= s.length <= 100`
* `s[i]` is either `'0'` or `'1'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let mut max0 = 0;
        let mut max1 = 0;
        let mut count = 0;
        let mut is0 = true;

        for c in s.chars() {
            match (c, is0) {
                ('0', true) | ('1', false) => count += 1,
                (_, true) => {
                    max0 = max0.max(count);
                    is0 = false;
                    count = 1;
                }
                (_, false) => {
                    max1 = max1.max(count);
                    is0 = true;
                    count = 1;
                }
            }
        }

        if is0 {
            max0 = max0.max(count);
        } else {
            max1 = max1.max(count);
        }

        max1 > max0
    }
}
```
