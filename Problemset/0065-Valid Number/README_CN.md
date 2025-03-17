# 65. 有效数字
给定一个字符串 `s` ，返回 `s` 是否是一个 **有效数字**。

例如，下面的都是有效数字：`"2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"`，而接下来的不是：`"abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"`。

一般的，一个 **有效数字** 可以用以下的规则之一定义：
1. 一个 **整数** 后面跟着一个 **可选指数**。
2. 一个 **十进制数** 后面跟着一个 **可选指数**。

一个 **整数** 定义为一个 **可选符号** `'-'` 或 `'+'` 后面跟着 **数字**。

一个 **十进制数** 定义为一个 **可选符号** `'-'` 或 `'+'` 后面跟着下述规则：
1. **数字** 后跟着一个 **小数点** `.`。
2. **数字** 后跟着一个 **小数点** `.` 再跟着 **数位**。
3. 一个 **小数点** `.` 后跟着 **数位**。

**指数** 定义为指数符号 `'e'` 或 `'E'`，后面跟着一个 **整数**。

**数字** 定义为一个或多个数位。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "0"
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "e"
<strong>输出:</strong> false
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "."
<strong>输出:</strong> false
</pre>

#### 提示:
* `1 <= s.length <= 20`
* `s` 仅含英文字母（大写和小写），数字（`0-9`），加号 `'+'` ，减号 `'-'` ，或者点 `'.'` 。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def isNumber(self, s: str) -> bool:
        def isDigits(s: str) -> bool:
            return s != "" and all(c in "0123456789" for c in s)

        def isInteger(s: str) -> bool:
            if s == "":
                return False

            if s[0] in "-+":
                s = s[1:]

            return isDigits(s)

        def isDecimal(s: str) -> bool:
            if s == "":
                return False

            if s[0] in "-+":
                s = s[1:]
            intpart, _, decipart = s.partition('.')

            if isDigits(intpart) and decipart == "":
                return True
            if isDigits(decipart) and intpart == "":
                return True

            return isDigits(intpart) and isDigits(decipart)

        num, e, exp = s.lower().partition('e')

        return (isInteger(num) or isDecimal(num)) and (e == "" or isInteger(exp))
```
