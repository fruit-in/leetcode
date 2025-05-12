# 301. Remove Invalid Parentheses
Given a string `s` that contains parentheses and letters, remove the minimum number of invalid parentheses to make the input string valid.

Return *a list of **unique strings** that are valid with the minimum number of removals*. You may return the answer in **any order**.

#### Example 1:
<pre>
<strong>Input:</strong> s = "()())()"
<strong>Output:</strong> ["(())()","()()()"]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "(a)())()"
<strong>Output:</strong> ["(a())()","(a)()()"]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = ")("
<strong>Output:</strong> [""]
</pre>

#### Constraints:
* `1 <= s.length <= 25`
* `s` consists of lowercase English letters and parentheses `'('` and `')'`.
* There will be at most `20` parentheses in `s`.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def removeInvalidParentheses(self, s: str) -> List[str]:
        used = [False] * len(s)
        maxlength = 0
        ret = {""}

        for i in range(len(s)):
            if s[i] not in "()":
                used[i] = True

        def dfs(i: int, opening: int, length: int) -> None:
            nonlocal maxlength, ret
            if i == len(s):
                if opening == 0:
                    if length > maxlength:
                        maxlength = length
                        ret = set()
                    if length == maxlength:
                        ret.add(''.join(s[i]
                                for i in range(len(s)) if used[i]))
                return
            if len(s) - i < opening or length + len(s) - i < maxlength:
                return

            if s[i] == '(':
                used[i] = True
                dfs(i + 1, opening + 1, length + 1)
                used[i] = False
                dfs(i + 1, opening, length)
            elif s[i] == ')':
                if opening > 0:
                    used[i] = True
                    dfs(i + 1, opening - 1, length + 1)
                    used[i] = False
                dfs(i + 1, opening, length)
            else:
                dfs(i + 1, opening, length + 1)

        dfs(0, 0, 0)

        return list(ret)
```
