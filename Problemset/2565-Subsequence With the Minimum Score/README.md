# 2565. Subsequence With the Minimum Score
You are given two strings `s` and `t`.

You are allowed to remove any number of characters from the string `t`.

The score of the string is `0` if no characters are removed from the string `t`, otherwise:
* Let `left` be the minimum index among all removed characters.
* Let `right` be the maximum index among all removed characters.

Then the score of the string is `right - left + 1`.

Return *the minimum possible score to make* `t` *a subsequence of* `s`.

A **subsequence** of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., `"ace"` is a subsequence of `"abcde"` while `"aec"` is not).

#### Example 1:
<pre>
<strong>Input:</strong> s = "abacaba", t = "bzaa"
<strong>Output:</strong> 1
<strong>Explanation:</strong> In this example, we remove the character "z" at index 1 (0-indexed).
The string t becomes "baa" which is a subsequence of the string "abacaba" and the score is 1 - 1 + 1 = 1.
It can be proven that 1 is the minimum score that we can achieve.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "cde", t = "xyz"
<strong>Output:</strong> 3
<strong>Explanation:</strong> In this example, we remove characters "x", "y" and "z" at indices 0, 1, and 2 (0-indexed).
The string t becomes "" which is a subsequence of the string "cde" and the score is 2 - 0 + 1 = 3.
It can be proven that 3 is the minimum score that we can achieve.
</pre>

#### Constraints:
* <code>1 <= s.length, t.length <= 10<sup>5</sup></code>
* `s` and `t` consist of only lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_score(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut i = t.len() - 1;
        let mut matches_right = vec![(t.len(), s.len())];
        let mut ret = t.len();

        for j in (0..s.len()).rev() {
            if s[j] == t[i] {
                matches_right.push((i, j));
                ret = i;

                if i == 0 {
                    break;
                } else {
                    i -= 1;
                }
            }
        }

        i = 0;

        for j in 0..s.len() {
            if s[j] == t[i] {
                while matches_right.last().unwrap_or(&(0, s.len())).1 <= j {
                    matches_right.pop();
                }
                ret = ret.min(
                    matches_right
                        .last()
                        .unwrap_or(&(t.len(), 0))
                        .0
                        .saturating_sub(i + 1),
                );

                if i == t.len() - 1 {
                    break;
                } else {
                    i += 1;
                }
            }
        }

        ret as i32
    }
}
```
