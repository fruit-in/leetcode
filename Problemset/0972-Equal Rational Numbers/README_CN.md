# 972. 相等的有理数
给定两个字符串 `s` 和 `t` ，每个字符串代表一个非负有理数，只有当它们表示相同的数字时才返回 `true` 。字符串中可以使用括号来表示有理数的重复部分。

**有理数** 最多可以用三个部分来表示：*整数部分* `<IntegerPart>`、*小数非重复部分* `<NonRepeatingPart>` 和*小数重复部分* `<(><RepeatingPart><)>`。数字可以用以下三种方法之一来表示：
* `<IntegerPart>`
    * 例： `0` ,`12` 和 `123`
* `<IntegerPart><.><NonRepeatingPart>`
    * 例： `0.5` , `1.` , `2.12` 和 `123.0001`
* `<IntegerPart><.><NonRepeatingPart><(><RepeatingPart><)>`
    * 例： `0.1(6)` ， `1.(9)`， `123.00(1212)`

十进制展开的重复部分通常在一对圆括号内表示。例如：
* `1 / 6 = 0.16666666... = 0.1(6) = 0.1666(6) = 0.166(66)`

#### 示例 1:
<pre>
<strong>输入:</strong> s = "0.(52)", t = "0.5(25)"
<strong>输出:</strong> true
<strong>解释:</strong> 因为 "0.(52)" 代表 0.52525252...，而 "0.5(25)" 代表 0.52525252525.....，则这两个字符串表示相同的数字。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "0.1666(6)", t = "0.166(66)"
<strong>输出:</strong> true
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "0.9(9)", t = "1."
<strong>输出:</strong> true
<strong>解释:</strong> "0.9(9)" 代表 0.999999999... 永远重复，等于 1 。[<a href="https://baike.baidu.com/item/0.999%E2%80%A6/5615429?fr=aladdin">有关说明，请参阅此链接</a>]
"1." 表示数字 1，其格式正确：(IntegerPart) = "1" 且 (NonRepeatingPart) = "" 。
</pre>

#### 提示:
* 每个部分仅由数字组成。
* 整数部分 `<IntegerPart>` 不会以零开头。（零本身除外）
* `1 <= <IntegerPart>.length <= 4`
* `0 <= <NonRepeatingPart>.length <= 4`
* `1 <= <RepeatingPart>.length <= 4`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def isRationalEqual(self, s: str, t: str) -> bool:
        def toFraction(s: str) -> (int, int):
            s = re.split(r"[.()]", s)
            integer = int(s[0])
            nonrepeating = int(s[1]) if len(s) > 1 and s[1] != '' else None
            repeating = int(s[2]) if len(s) > 2 else None

            if nonrepeating is None and repeating is None:
                denominator = 1
                numerator = 0
            elif nonrepeating is None:
                denominator = 10 ** len(s[2]) - 1
                numerator = repeating
            elif repeating is None:
                denominator = 10 ** len(s[1])
                numerator = nonrepeating
            else:
                denominator = 10 ** (len(s[1]) + len(s[2])) - 10 ** (len(s[1]))
                numerator = nonrepeating * (10 ** len(s[2]) - 1) + repeating

            numerator += denominator * integer
            gcdnum = gcd(numerator, denominator)

            return (numerator // gcdnum, denominator // gcdnum)

        return toFraction(s) == toFraction(t)
```
