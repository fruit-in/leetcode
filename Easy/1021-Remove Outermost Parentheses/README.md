# 1021. Remove Outermost Parentheses
A valid parentheses string is either empty <code>("")</code>, <code>"(" + A + ")"</code>, or <code>A + B</code>, where <code>A</code> and <code>B</code> are valid parentheses strings, and <code>+</code> represents string concatenation.  For example, <code>""</code>, <code>"()"</code>, <code>"(())()"</code>, and <code>"(()(()))"</code> are all valid parentheses strings.

A valid parentheses string <code>S</code> is **primitive** if it is nonempty, and there does not exist a way to split it into <code>S = A+B</code>, with <code>A</code> and <code>B</code> nonempty valid parentheses strings.

Given a valid parentheses string <code>S</code>, consider its primitive decomposition: <code>S = P_1 + P_2 + ... + P_k</code>, where <code>P_i</code> are primitive valid parentheses strings.

Return <code>S</code> after removing the outermost parentheses of every primitive string in the primitive decomposition of <code>S</code>.

#### Example 1:
<pre>
<strong>Input:</strong> "(()())(())"
<strong>Output:</strong> "()()()"
<strong>Explanation:</strong> 
The input string is "(()())(())", with primitive decomposition "(()())" + "(())".
After removing outer parentheses of each part, this is "()()" + "()" = "()()()".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "(()())(())(()(()))"
<strong>Output:</strong> "()()()()(())"
<strong>Explanation:</strong> 
The input string is "(()())(())(()(()))", with primitive decomposition "(()())" + "(())" + "(()(()))".
After removing outer parentheses of each part, this is "()()" + "()" + "()(())" = "()()()()(())".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> "()()"
<strong>Output:</strong> ""
<strong>Explanation:</strong> 
The input string is "()()", with primitive decomposition "()" + "()".
After removing outer parentheses of each part, this is "" + "" = "".
</pre>

#### Note:
1. <code>S.length <= 10000</code>
2. <code>S[i]</code> is <code>"("</code> or <code>")"</code>
3. <code>S</code> is a valid parentheses string

## Solutions

### 1. Solution (Rust)
```Rust
impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut counter = 0;
        let mut result = String::new();
        for ch in s.chars() {
            if ch == ')' {
                counter += 1;
            }
            if counter != 0 {
                result.push(ch);
            }
            if ch == '(' {
                counter -= 1;
            }
        }
        result
    }
}
```
