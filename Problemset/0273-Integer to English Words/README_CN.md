# 273. 整数转换英文表示
将非负整数 `num` 转换为其对应的英文表示。

#### 示例 1:
<pre>
<strong>输入:</strong> num = 123
<strong>输出:</strong> "One Hundred Twenty Three"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num = 12345
<strong>输出:</strong> "Twelve Thousand Three Hundred Forty Five"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> num = 1234567
<strong>输出:</strong> "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
</pre>

#### 提示:
* <code>0 <= num <= 2<sup>31</sup> - 1</code>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def numberToWords(self, num: int) -> str:
        if num == 0:
            return "Zero"

        billion = num // 1000000000
        million = num % 1000000000 // 1000000
        thousand = num % 1000000 // 1000
        lt1000 = num % 1000
        ret = []

        if billion > 0:
            ret.append(self.numberToWordsLT1000(billion) + " Billion")
        if million > 0:
            ret.append(self.numberToWordsLT1000(million) + " Million")
        if thousand > 0:
            ret.append(self.numberToWordsLT1000(thousand) + " Thousand")
        if lt1000 > 0:
            ret.append(self.numberToWordsLT1000(lt1000))

        return ' '.join(ret)

    def numberToWordsLT1000(self, num: int) -> str:
        lt20 = [
            "Zero", "One", "Two", "Three", "Four",
            "Five", "Six", "Seven", "Eight", "Nine",
            "Ten", "Eleven", "Twelve", "Thirteen", "Fourteen",
            "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen",
        ]
        tens = [
            "Zero", "Ten", "Twenty", "Thirty", "Forty",
            "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
        ]

        hundred = num // 100
        ten = num % 100 // 10
        unit = num % 10
        ret = []

        if hundred > 0:
            ret.append(lt20[hundred] + " Hundred")
        if ten > 1:
            ret.append(tens[ten])
        if ten == 1:
            ret.append(lt20[10 + unit])
        elif unit > 0:
            ret.append(lt20[unit])

        return ' '.join(ret)
```
