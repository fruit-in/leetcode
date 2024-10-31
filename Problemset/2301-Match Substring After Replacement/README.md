# 2301. Match Substring After Replacement
You are given two strings `s` and `sub`. You are also given a 2D character array `mappings` where <code>mappings[i] = [old<sub>i</sub>, new<sub>i</sub>]</code> indicates that you may perform the following operation **any** number of times:

* **Replace** a character <code>old<sub>i</sub></code> of `sub` with <code>new<sub>i</sub></code>.

Each character in `sub` **cannot** be replaced more than once.

Return `true` *if it is possible to make* `sub` *a substring of* `s` *by replacing zero or more characters according to* `mappings`. Otherwise, return `false`.

A **substring** is a contiguous non-empty sequence of characters within a

#### Example 1:
<pre>
<strong>Input:</strong> s = "fool3e7bar", sub = "leet", mappings = [["e","3"],["t","7"],["t","8"]]
<strong>Output:</strong> true
<strong>Explanation:</strong> Replace the first 'e' in sub with '3' and 't' in sub with '7'.
Now sub = "l3e7" is a substring of s, so we return true.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "fooleetbar", sub = "f00l", mappings = [["o","0"]]
<strong>Output:</strong> false
<strong>Explanation:</strong> The string "f00l" is not a substring of s and no replacements can be made.
Note that we cannot replace '0' with 'o'.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "Fool33tbaR", sub = "leetd", mappings = [["e","3"],["t","7"],["t","8"],["d","b"],["p","b"]]
<strong>Output:</strong> true
<strong>Explanation:</strong> Replace the first and second 'e' in sub with '3' and 'd' in sub with 'b'.
Now sub = "l33tb" is a substring of s, so we return true.
</pre>

#### Constraints:
* `1 <= sub.length <= s.length <= 5000`
* `0 <= mappings.length <= 1000`
* `mappings[i].length == 2`
* <code>old<sub>i</sub> != new<sub>i</sub></code>
* `s` and `sub` consist of uppercase and lowercase English letters and digits.
* <code>old<sub>i</sub></code> and <code>new<sub>i</sub></code> are either uppercase or lowercase English letters or digits.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn match_replacement(s: String, sub: String, mappings: Vec<Vec<char>>) -> bool {
        let s = s.as_bytes();
        let sub = sub.as_bytes();
        let mut bitmappings = [0_i128; 128];

        for i in 0..mappings.len() {
            let (old, new) = (mappings[i][0] as usize, mappings[i][1] as usize);
            bitmappings[old] |= 1 << new;
        }

        for i in 0..=s.len() - sub.len() {
            let mut flag = true;

            for j in 0..sub.len() {
                if sub[j] != s[i + j] && bitmappings[sub[j] as usize] & (1 << s[i + j]) == 0 {
                    flag = false;
                    break;
                }
            }

            if flag {
                return true;
            }
        }

        false
    }
}
```
