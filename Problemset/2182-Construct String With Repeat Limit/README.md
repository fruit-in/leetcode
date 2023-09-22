# 2182. Construct String With Repeat Limit
You are given a string `s` and an integer `repeatLimit`. Construct a new string `repeatLimitedString` using the characters of `s` such that no letter appears **more than** `repeatLimit` times **in a row**. You do **not** have to use all characters from `s`.

Return *the **lexicographically largest*** `repeatLimitedString` *possible*.

A string `a` is **lexicographically** larger than a string `b` if in the first position where `a` and `b` differ, string `a` has a letter that appears later in the alphabet than the corresponding letter in `b`. If the first `min(a.length, b.length)` characters do not differ, then the longer string is the lexicographically larger one.

#### Example 1:
<pre>
<strong>Input:</strong> s = "cczazcc", repeatLimit = 3
<strong>Output:</strong> "zzcccac"
<strong>Explanation:</strong> We use all of the characters from s to construct the repeatLimitedString "zzcccac".
The letter 'a' appears at most 1 time in a row.
The letter 'c' appears at most 3 times in a row.
The letter 'z' appears at most 2 times in a row.
Hence, no letter appears more than repeatLimit times in a row and the string is a valid repeatLimitedString.
The string is the lexicographically largest repeatLimitedString possible so we return "zzcccac".
Note that the string "zzcccca" is lexicographically larger but the letter 'c' appears more than 3 times in a row, so it is not a valid repeatLimitedString.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "aababab", repeatLimit = 2
<strong>Output:</strong> "bbabaa"
<strong>Explanation:</strong> We use only some of the characters from s to construct the repeatLimitedString "bbabaa".
The letter 'a' appears at most 2 times in a row.
The letter 'b' appears at most 2 times in a row.
Hence, no letter appears more than repeatLimit times in a row and the string is a valid repeatLimitedString.
The string is the lexicographically largest repeatLimitedString possible so we return "bbabaa".
Note that the string "bbabaaa" is lexicographically larger but the letter 'a' appears more than 2 times in a row, so it is not a valid repeatLimitedString.
</pre>

#### Constraints:
* <code>1 <= repeatLimit <= s.length <= 10<sup>5</sup></code>
* `s` consists of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut count = vec![0; 26];
        let mut repeat = 0;
        let mut flag = true;
        let mut ret = vec![];

        for c in s.bytes() {
            count[(c - b'a') as usize] += 1;
        }

        while flag {
            flag = false;

            for c in (b'a'..=b'z').rev() {
                if count[(c - b'a') as usize] > 0 {
                    if *ret.last().unwrap_or(&b' ') != c || repeat < repeat_limit {
                        if *ret.last().unwrap_or(&b' ') != c {
                            repeat = 0;
                        }
                        count[(c - b'a') as usize] -= 1;
                        repeat += 1;
                        flag = true;
                        ret.push(c);
                        break;
                    }
                }
            }
        }

        String::from_utf8(ret).unwrap()
    }
}
```
