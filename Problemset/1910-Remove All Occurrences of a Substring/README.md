# 1910. Remove All Occurrences of a Substring
Given two strings `s` and `part`, perform the following operation on `s` until **all** occurrences of the substring `part` are removed:
* Find the **leftmost** occurrence of the substring `part` and **remove** it from `s`.

Return `s` *after removing all occurrences of* `part`.

A **substring** is a contiguous sequence of characters in a string.

#### Example 1:
<pre>
<strong>Input:</strong> s = "daabcbaabcbc", part = "abc"
<strong>Output:</strong> "dab"
<strong>Explanation:</strong> The following operations are done:
- s = "daabcbaabcbc", remove "abc" starting at index 2, so s = "dabaabcbc".
- s = "dabaabcbc", remove "abc" starting at index 4, so s = "dababc".
- s = "dababc", remove "abc" starting at index 3, so s = "dab".
Now s has no occurrences of "abc".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "axxxxyyyyb", part = "xy"
<strong>Output:</strong> "ab"
<strong>Explanation:</strong> The following operations are done:
- s = "axxxxyyyyb", remove "xy" starting at index 4 so s = "axxxyyyb".
- s = "axxxyyyb", remove "xy" starting at index 3 so s = "axxyyb".
- s = "axxyyb", remove "xy" starting at index 2 so s = "axyb".
- s = "axyb", remove "xy" starting at index 1 so s = "ab".
Now s has no occurrences of "xy".
</pre>

#### Constraints:
* `1 <= s.length <= 1000`
* `1 <= part.length <= 1000`
* `s` and `part` consists of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut s = s;

        while s.contains(&part) {
            s = s.replacen(&part, "", 1);
        }

        s
    }
}
```
