# 1849. 将字符串拆分为递减的连续值
给你一个仅由数字组成的字符串 `s` 。

请你判断能否将 `s` 拆分成两个或者多个 **非空子字符串** ，使子字符串的 **数值** 按 **降序** 排列，且每两个 **相邻子字符串** 的数值之 **差** 等于 `1` 。

* 例如，字符串 `s = "0090089"` 可以拆分成 `["0090", "089"]` ，数值为 `[90,89]` 。这些数值满足按降序排列，且相邻值相差 `1` ，这种拆分方法可行。
* 另一个例子中，字符串 `s = "001"` 可以拆分成 `["0", "01"]`、`["00", "1"]` 或 `["0", "0", "1"]` 。然而，所有这些拆分方法都不可行，因为对应数值分别是 `[0,1]`、`[0,1]` 和 `[0,0,1]` ，都不满足按降序排列的要求。

如果可以按要求拆分 `s` ，返回 `true` ；否则，返回 `false` 。

**子字符串** 是字符串中的一个连续字符序列。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "1234"
<strong>输出:</strong> false
<strong>解释:</strong> 不存在拆分 s 的可行方法。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "050043"
<strong>输出:</strong> true
<strong>解释:</strong> s 可以拆分为 ["05", "004", "3"] ，对应数值为 [5,4,3] 。
满足按降序排列，且相邻值相差 1 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "9080701"
<strong>输出:</strong> false
<strong>解释:</strong> 不存在拆分 s 的可行方法。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s = "10009998"
<strong>输出:</strong> true
<strong>解释:</strong> s 可以拆分为 ["100", "099", "98"] ，对应数值为 [100,99,98] 。
满足按降序排列，且相邻值相差 1 。
</pre>

#### 提示:
* `1 <= s.length <= 20`
* `s` 仅由数字组成

## 题解 (Python)

### 1. 题解
```Python
from functools import cache


class Solution:
    @cache
    def splitString(self, s: str, x: int = None) -> bool:
        if x is None:
            return any(self.splitString(s[i:], int(s[:i])) for i in range(1, len(s)))

        if s == "":
            return True

        return any(x - 1 == int(s[:i]) and self.splitString(s[i:], int(s[:i])) for i in range(1, len(s) + 1))
```
