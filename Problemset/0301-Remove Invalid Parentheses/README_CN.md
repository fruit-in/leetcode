# 301. 删除无效的括号
给你一个由若干括号和字母组成的字符串 `s` ，删除最小数量的无效括号，使得输入的字符串有效。

返回所有可能的结果。答案可以按 **任意顺序** 返回。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "()())()"
<strong>输出:</strong> ["(())()","()()()"]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "(a)())()"
<strong>输出:</strong> ["(a())()","(a)()()"]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = ")("
<strong>输出:</strong> [""]
</pre>

#### 提示:
* `1 <= s.length <= 25`
* `s` 由小写英文字母以及括号 `'('` 和 `')'` 组成
* `s` 中至多含 `20` 个括号

## 题解 (Python)

### 1. 题解
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
