# 880. Decoded String at Index
You are given an encoded string `s`. To decode the string to a tape, the encoded string is read one character at a time and the following steps are taken:

* If the character read is a letter, that letter is written onto the tape.
* If the character read is a digit `d`, the entire current tape is repeatedly written `d - 1` more times in total.

Given an integer `k`, return *the* <code>k<sup>th</sup></code> *letter (**1-indexed**) in the decoded string*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "leet2code3", k = 10
<strong>Output:</strong> "o"
<strong>Explanation:</strong> The decoded string is "leetleetcodeleetleetcodeleetleetcode".
The 10th letter in the string is "o".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "ha22", k = 5
<strong>Output:</strong> "h"
<strong>Explanation:</strong> The decoded string is "hahahaha".
The 5th letter is "h".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "a2345678999999999999999", k = 1
<strong>Output:</strong> "a"
<strong>Explanation:</strong> The decoded string is "a" repeated 8301530446056247680 times.
The 1st letter is "a".
</pre>

#### Constraints:
* `2 <= s.length <= 100`
* `s` consists of lowercase English letters and digits `2` through `9`.
* `s` starts with a letter.
* <code>1 <= k <= 10<sup>9</sup></code>
* It is guaranteed that `k` is less than or equal to the length of the decoded string.
* The decoded string is guaranteed to have less than <code>2<sup>63</sup></code> letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut k = k as i64 - 1;
        let mut chars = vec![];
        let mut length = 0;

        for ch in s.bytes() {
            chars.push((ch, length));

            if ch.is_ascii_lowercase() {
                length += 1;
            } else {
                length *= (ch - b'0') as i64;
            }

            if length > k {
                break;
            }
        }

        while let Some((ch, i)) = chars.pop() {
            if ch.is_ascii_lowercase() {
                if i == k {
                    return String::from_utf8(vec![ch]).unwrap();
                }

                length -= 1;
            } else {
                length /= (ch - b'0') as i64;
                k %= length;
            }
        }

        unreachable!()
    }
}
```
