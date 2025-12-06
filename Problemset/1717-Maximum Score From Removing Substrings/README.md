# 1717. Maximum Score From Removing Substrings
You are given a string `s` and two integers `x` and `y`. You can perform two types of operations any number of times.

* Remove substring `"ab"` and gain `x` points.
    * For example, when removing `"ab"` from `"cabxbae"` it becomes `"cxbae"`.
* Remove substring `"ba"` and gain `y` points.
    * For example, when removing `"ba"` from `"cabxbae"` it becomes `"cabxe"`.

Return *the maximum points you can gain after applying the above operations on* `s`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "cdbcbbaaabab", x = 4, y = 5
<strong>Output:</strong> 19
<strong>Explanation:</strong>
- Remove the "ba" underlined in "cdbcbbaaabab". Now, s = "cdbcbbaaab" and 5 points are added to the score.
- Remove the "ab" underlined in "cdbcbbaaab". Now, s = "cdbcbbaa" and 4 points are added to the score.
- Remove the "ba" underlined in "cdbcbbaa". Now, s = "cdbcba" and 5 points are added to the score.
- Remove the "ba" underlined in "cdbcba". Now, s = "cdbc" and 5 points are added to the score.
Total score = 5 + 4 + 5 + 5 = 19.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "aabbaaxybbaabb", x = 5, y = 4
<strong>Output:</strong> 20
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* <code>1 <= x, y <= 10<sup>4</sup></code>
* `s` consists of lowercase English letters.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        if x < y {
            return Self::maximum_gain(
                s.replace("a", "$").replace("b", "a").replace("$", "b"),
                y,
                x,
            );
        }

        let mut stack1 = vec![];
        let mut ret = 0;

        for c in s.chars().chain(std::iter::once('c')) {
            if c == 'b' && stack1.last() == Some(&'a') {
                stack1.pop();
                ret += x;
            } else if c == 'a' || c == 'b' {
                stack1.push(c);
            } else {
                let mut stack2 = vec![];

                for c in stack1.drain(..) {
                    if c == 'a' && stack2.last() == Some(&'b') {
                        stack2.pop();
                        ret += y;
                    } else {
                        stack2.push(c);
                    }
                }
            }
        }

        ret
    }
}
```
