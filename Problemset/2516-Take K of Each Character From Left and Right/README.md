# 2516. Take K of Each Character From Left and Right
You are given a string `s` consisting of the characters `'a'`, `'b'`, and `'c'` and a non-negative integer `k`. Each minute, you may take either the **leftmost** character of `s`, or the **rightmost** character of `s`.

Return *the **minimum** number of minutes needed for you to take **at least*** `k` *of each character, or return* `-1` *if it is not possible to take* `k` *of each character*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "aabaaaacaabc", k = 2
<strong>Output:</strong> 8
<strong>Explanation:</strong>
Take three characters from the left of s. You now have two 'a' characters, and one 'b' character.
Take five characters from the right of s. You now have four 'a' characters, two 'b' characters, and two 'c' characters.
A total of 3 + 5 = 8 minutes is needed.
It can be proven that 8 is the minimum number of minutes needed.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "a", k = 1
<strong>Output:</strong> -1
<strong>Explanation:</strong> It is not possible to take one 'b' or 'c' so return -1.
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` consists of only the letters `'a'`, `'b'`, and `'c'`.
* `0 <= k <= s.length`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut count_a = 0;
        let mut count_b = 0;
        let mut count_c = 0;
        let mut i = 0;
        let mut ret: usize;

        while i < n && (count_a < k || count_b < k || count_c < k) {
            count_a += (s[i] == b'a') as i32;
            count_b += (s[i] == b'b') as i32;
            count_c += (s[i] == b'c') as i32;
            i += 1;
        }

        if count_a < k || count_b < k || count_c < k {
            return -1;
        }

        ret = i;

        for j in 1..n {
            count_a += (s[n - j] == b'a') as i32;
            count_b += (s[n - j] == b'b') as i32;
            count_c += (s[n - j] == b'c') as i32;

            while i > 0
                && count_a - (s[i - 1] == b'a') as i32 >= k
                && count_b - (s[i - 1] == b'b') as i32 >= k
                && count_c - (s[i - 1] == b'c') as i32 >= k
            {
                count_a -= (s[i - 1] == b'a') as i32;
                count_b -= (s[i - 1] == b'b') as i32;
                count_c -= (s[i - 1] == b'c') as i32;
                i -= 1;
            }

            if count_a >= k && count_b >= k && count_c >= k {
                ret = ret.min(i + j);
            }
        }

        ret as i32
    }
}
```
