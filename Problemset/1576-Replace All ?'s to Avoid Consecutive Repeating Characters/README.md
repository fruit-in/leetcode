# 1576. Replace All ?'s to Avoid Consecutive Repeating Characters
Given a string `s` containing only lower case English letters and the '?' character, convert **all** the '?' characters into lower case letters such that the final string does not contain any **consecutive repeating** characters. You **cannot** modify the non '?' characters.

It is **guaranteed** that there are no consecutive repeating characters in the given string **except** for '?'.

Return the final string after all the conversions (possibly zero) have been made. If there is more than one solution, return any of them. It can be shown that an answer is always possible with the given constraints.

#### Example 1:
<pre>
<b>Input:</b> s = "?zs"
<b>Output:</b> "azs"
<b>Explanation:</b> There are 25 solutions for this problem. From "azs" to "yzs", all are valid. Only "z" is an invalid modification as the string will consist of consecutive repeating characters in "zzs".
</pre>

#### Example 2:
<pre>
<b>Input:</b> s = "ubv?w"
<b>Output:</b> "ubvaw"
<b>Explanation:</b> There are 24 solutions for this problem. Only "v" and "w" are invalid modifications as the strings will consist of consecutive repeating characters in "ubvvw" and "ubvww".
</pre>

#### Example 3:
<pre>
<b>Input:</b> s = "j?qg??b"
<b>Output:</b> "jaqgacb"
</pre>

#### Example 4:
<pre>
<b>Input:</b> s = "??yw?ipkj?"
<b>Output:</b> "acywaipkja"
</pre>

#### Constraints:
* `1 <= s.length <= 100`
* `s` contains only lower case English letters and `'?'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut s = s.into_bytes();

        for i in 0..s.len() {
            if s[i] == b'?' {
                if i == 0 {
                    if s.len() == 1 || s[1] != b'a' {
                        s[0] = b'a';
                    } else {
                        s[0] = b'b';
                    }
                } else {
                    s[i] = s[i - 1] % 26 + b'a';
                    if i + 1 < s.len() && s[i + 1] == s[i] {
                        s[i] = s[i] % 26 + b'a';
                    }
                }
            }
        }

        String::from_utf8(s).unwrap()
    }
}
```
