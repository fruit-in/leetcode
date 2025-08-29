# 678. Valid Parenthesis String
Given a string `s` containing only three types of characters: `'('`, `')'` and `'*'`, return `true` *if* `s` *is **valid***.

The following rules define a **valid** string:
* Any left parenthesis `'('` must have a corresponding right parenthesis `')'`.
* Any right parenthesis `')'` must have a corresponding left parenthesis `'('`.
* Left parenthesis `'('` must go before the corresponding right parenthesis `')'`.
* `'*'` could be treated as a single right parenthesis `')'` or a single left parenthesis `'('` or an empty string `""`.

#### Example 1:
<pre>
<strong>Input:</strong> s = "()"
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "(*)"
<strong>Output:</strong> true
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "(*))"
<strong>Output:</strong> true
</pre>

#### Constraints:
* `1 <= s.length <= 100`
* `s[i]` is `'('`, `')'` or `'*'`.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut possible_counts = HashSet::from([0]);

        for c in s.chars() {
            let mut next_counts = HashSet::new();

            for count in possible_counts.into_iter() {
                if c == '(' {
                    next_counts.insert(count + 1);
                } else if c == ')' && count > 0 {
                    next_counts.insert(count - 1);
                } else if c == '*' {
                    next_counts.insert(count);
                    next_counts.insert(count + 1);
                    next_counts.insert((count - 1).max(0));
                }
            }

            possible_counts = next_counts;
        }

        possible_counts.contains(&0)
    }
}
```
