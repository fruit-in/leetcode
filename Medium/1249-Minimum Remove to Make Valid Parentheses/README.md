# 1249. Minimum Remove to Make Valid Parentheses
Given a string `s` of `'('` , `')'` and lowercase English characters.

Your task is to remove the minimum number of parentheses ( `'('` or `')'`, in any positions ) so that the resulting *parentheses string* is valid and return **any** valid string.

Formally, a *parentheses string* is valid if and only if:
* It is the empty string, contains only lowercase characters, or
* It can be written as `AB` (`A` concatenated with `B`), where `A` and `B` are valid strings, or
* It can be written as `(A)`, where `A` is a valid string.

#### Example 1:
<pre>
<b>Input:</b> s = "lee(t(c)o)de)"
<b>Output:</b> "lee(t(c)o)de"
<b>Explanation:</b> "lee(t(co)de)" , "lee(t(c)ode)" would also be accepted.
</pre>

#### Example 2:
<pre>
<b>Input:</b> s = "a)b(c)d"
<b>Output:</b> "ab(c)d"
</pre>

#### Example 3:
<pre>
<b>Input:</b> s = "))(("
<b>Output:</b> ""
<b>Explanation:</b> An empty string is also valid.
</pre>

#### Example 4:
<pre>
<b>Input:</b> s = "(a(b(c)d)"
<b>Output:</b> "a(b(c)d)"
</pre>

#### Constraints:
* `1 <= s.length <= 10^5`
* `s[i]` is one of  `'('` , `')'` and lowercase English letters.

## Solutions (Rust)

### 1. Stack
```Rust
impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut lp = vec![];
        let mut ret = vec![];

        for ch in s.bytes() {
            if ch != b')' || lp.len() > 0 {
                ret.push(ch);
            }
            if ch == b'(' {
                lp.push(ret.len() - 1);
            } else if ch == b')' {
                lp.pop();
            }
        }

        while let Some(i) = lp.pop() {
            ret.remove(i);
        }

        String::from_utf8(ret).unwrap()
    }
}
```
