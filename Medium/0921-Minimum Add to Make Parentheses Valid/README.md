# 921. Minimum Add to Make Parentheses Valid
Given a string <code>S</code> of <code>'('</code> and <code>')'</code> parentheses, we add the minimum number of parentheses ( <code>'('</code> or <code>')'</code>, and in any positions ) so that the resulting parentheses string is valid.

Formally, a parentheses string is valid if and only if:
* It is the empty string, or
* It can be written as <code>AB</code> (<code>A</code> concatenated with <code>B</code>), where <code>A</code> and <code>B</code> are valid strings, or
* It can be written as <code>(A)</code>, where <code>A</code> is a valid string.

Given a parentheses string, return the minimum number of parentheses we must add to make the resulting string valid.

#### Example 1:
<pre>
<strong>Input:</strong> "())"
<strong>Output:</strong> 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "((("
<strong>Output:</strong> 3
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> "()"
<strong>Output:</strong> 0
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> "()))(("
<strong>Output:</strong> 4
</pre>

#### Note:
1. <code>S.length <= 1000</code>
2. S only consists of <code>'('</code> and <code>')'</code> characters.

## Solutions

### 1. Remove Valid Parentheses from String (Python3)
```Python3
class Solution:
    def minAddToMakeValid(self, S: str) -> int:
        while S.count("()"):
            S = S.replace("()", "")
        return len(S)
```

### 2. Remove Valid Parentheses by Stack (Rust)
```Rust
impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut stack: Vec<char> = Vec::new();
        for ch in s.chars() {
            if ch == ')' && stack.ends_with(&['(']) {
                stack.pop();
            } else {
                stack.push(ch);
            }
        }
        stack.len() as i32
    }
}
```

### 3. Balance (Rust)
```Rust
impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut left = 0;
        let mut right = 0;
        for ch in s.chars() {
            if ch == '(' {
                left += 1
            } else if left > 0{
                left -= 1
            } else {
                right += 1
            }
        }
        left + right
    }
}
```
