# 1190. Reverse Substrings Between Each Pair of Parentheses
You are given a string `s` that consists of lower case English letters and brackets.

Reverse the strings in each pair of matching parentheses, starting from the innermost one.

Your result should **not** contain any brackets.

#### Example 1:
<pre>
<strong>Input:</strong> s = "(abcd)"
<strong>Output:</strong> "dcba"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "(u(love)i)"
<strong>Output:</strong> "iloveu"
<strong>Explanation:</strong> The substring "love" is reversed first, then the whole string is reversed.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "(ed(et(oc))el)"
<strong>Output:</strong> "leetcode"
<strong>Explanation:</strong> First, we reverse the substring "oc", then "etco", and finally, the whole string.
</pre>

#### Constraints:
* `1 <= s.length <= 2000`
* `s` only contains lower case English characters and parentheses.
* It is guaranteed that all parentheses are balanced.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut ret = vec![];

        for ch0 in s.bytes() {
            if ch0 == b')' {
                let mut stack = vec![];

                while let Some(ch1) = ret.pop() {
                    if ch1 == b'(' {
                        break;
                    }
                    stack.push(ch1);
                }

                ret.append(&mut stack);
            } else {
                ret.push(ch0);
            }
        }

        String::from_utf8(ret).unwrap()
    }
}
```
