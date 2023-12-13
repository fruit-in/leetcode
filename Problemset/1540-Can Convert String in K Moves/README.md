# 1540. Can Convert String in K Moves
Given two strings `s` and `t`, your goal is to convert `s` into `t` in `k` moves or less.

During the <code>i<sup>th</sup></code> (`1 <= i <= k`) move you can:

* Choose any index `j` (1-indexed) from `s`, such that `1 <= j <= s.length` and `j` has not been chosen in any previous move, and shift the character at that index `i` times.
* Do nothing.

Shifting a character means replacing it by the next letter in the alphabet (wrapping around so that `'z'` becomes `'a'`). Shifting a character by `i` means applying the shift operations `i` times.

Remember that any index `j` can be picked at most once.

Return `true` if it's possible to convert `s` into `t` in no more than `k` moves, otherwise return `false`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "input", t = "ouput", k = 9
<strong>Output:</strong> true
<strong>Explanation:</strong> In the 6th move, we shift 'i' 6 times to get 'o'. And in the 7th move we shift 'n' to get 'u'.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "abc", t = "bcd", k = 10
<strong>Output:</strong> false
<strong>Explanation:</strong> We need to shift each character in s one time to convert it into t. We can shift 'a' to 'b' during the 1st move. However, there is no way to shift the other characters in the remaining moves to obtain t from s.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "aab", t = "bbb", k = 27
<strong>Output:</strong> true
<strong>Explanation:</strong> In the 1st move, we shift the first 'a' 1 time to get 'b'. In the 27th move, we shift the second 'a' 27 times to get 'b'.
</pre>

#### Constraints:
* `1 <= s.length, t.length <= 10^5`
* `0 <= k <= 10^9`
* `s`, `t` contain only lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn can_convert_string(s: String, t: String, k: i32) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut count = [0; 26];

        for (ch0, ch1) in s.bytes().zip(t.bytes()) {
            count[(ch1 + 26 - ch0) as usize % 26] += 1;
        }

        (1..26).all(|x| 26 * (count[x] - 1) + x as i32 <= k)
    }
}
```
