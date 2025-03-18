# 394. Decode String
Given an encoded string, return its decoded string.

The encoding rule is: `k[encoded_string]`, where the `encoded_string` inside the square brackets is being repeated exactly `k` times. Note that `k` is guaranteed to be a positive integer.

You may assume that the input string is always valid; there are no extra white spaces, square brackets are well-formed, etc. Furthermore, you may assume that the original data does not contain any digits and that digits are only for those repeat numbers, `k`. For example, there will not be input like `3a` or `2[4]`.

The test cases are generated so that the length of the output will never exceed <code>10<sup>5</sup></code>.

#### Example 1:
<pre>
<strong>Input:</strong> s = "3[a]2[bc]"
<strong>Output:</strong> "aaabcbc"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "3[a2[c]]"
<strong>Output:</strong> "accaccacc"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "2[abc]3[cd]ef"
<strong>Output:</strong> "abcabccdcdcdef"
</pre>

#### Constraints:
* `1 <= s.length <= 30`
* `s` consists of lowercase English letters, digits, and square brackets `'[]'`.
* `s` is guaranteed to be **a valid** input.
* All the integers in `s` are in the range `[1, 300]`.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def decodeString(self, s: str) -> str:
        stack = []
        k = 0

        for c in s:
            if c.isdigit():
                k = k * 10 + int(c)
            elif c == '[':
                stack.append(str(k))
                k = 0
            elif c.islower():
                stack.append(c)
            elif c == ']':
                tmp = []
                while stack[-1].islower():
                    tmp.append(stack.pop())
                stack.append(''.join(tmp[::-1]) * int(stack.pop()))

        return ''.join(stack)
```
