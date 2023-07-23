# 273. Integer to English Words
Convert a non-negative integer `num` to its English words representation.

#### Example 1:
<pre>
<strong>Input:</strong> num = 123
<strong>Output:</strong> "One Hundred Twenty Three"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = 12345
<strong>Output:</strong> "Twelve Thousand Three Hundred Forty Five"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> num = 1234567
<strong>Output:</strong> "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven"
</pre>

#### Constraints:
* <code>0 <= num <= 2<sup>31</sup> - 1</code>

## Solutions (Python)

### 1. Solution
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
