# 12. 整数转罗马数字
罗马数字包含以下七种字符： ```I```， ```V```， ```X```， ```L```，```C```，```D``` 和 ```M```。

<pre>
<strong>字符</strong>          <strong>数值</strong>
I             1
V             5
X             10
L             50
C             100
D             500
M             1000
</pre>

例如， 罗马数字 2 写做 ```II``` ，即为两个并列的 1。12 写做 ```XII``` ，即为 ```X``` + ```II``` 。 27 写做  ```XXVII```, 即为 ```XX``` + ```V``` + ```II``` 。

通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 ```IIII```，而是 ```IV```。数字 1 在数字 5 的左边，所表示的数等于大数 5 减小数 1 得到的数值 4 。同样地，数字 9 表示为 ```IX```。这个特殊的规则只适用于以下六种情况：
* ```I``` 可以放在 ```V``` (5) 和 ```X``` (10) 的左边，来表示 4 和 9。
* ```X``` 可以放在 ```L``` (50) 和 ```C``` (100) 的左边，来表示 40 和 90。
* ```C``` 可以放在 ```D``` (500) 和 ```M``` (1000) 的左边，来表示 400 和 900。

给定一个整数，将其转为罗马数字。输入确保在 1 到 3999 的范围内。

#### 示例 1:
<pre>
<strong>输入:</strong> 3
<strong>输出:</strong> "III"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 4
<strong>输出:</strong> "IV"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> 9
<strong>输出:</strong> "IX"
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> 58
<strong>输出:</strong> "LVIII"
<strong>解释:</strong> L = 50, V = 5, III = 3.
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> 1994
<strong>输出:</strong> "MCMXCIV"
<strong>解释:</strong> M = 1000, CM = 900, XC = 90 and IV = 4.
</pre>

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def intToRoman(self, num: int) -> str:
        roman = (num // 1000) * 'M'
        roman += (num % 1000 // 500) * 'D'
        roman += (num % 500 // 100) * 'C'
        roman += (num % 100 // 50) * 'L'
        roman += (num % 50 // 10) * 'X'
        roman += (num % 10 // 5) * 'V'
        roman += (num % 5) * 'I'

        roman = roman.replace("DCCCC", "CM")
        roman = roman.replace("CCCC", "CD")
        roman = roman.replace("LXXXX", "XC")
        roman = roman.replace("XXXX", "XL")
        roman = roman.replace("VIIII", "IX")
        roman = roman.replace("IIII", "IV")

        return roman
```

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer} num
# @return {String}
def int_to_roman(num)
    roman = 'M' * (num / 1000)
    roman += 'D' * (num % 1000 / 500)
    roman += 'C' * (num % 500 / 100)
    roman += 'L' * (num % 100 / 50)
    roman += 'X' * (num % 50 / 10)
    roman += 'V' * (num % 10 / 5)
    roman += 'I' * (num % 5)

    roman.sub!("DCCCC", "CM")
    roman.sub!("CCCC", "CD")
    roman.sub!("LXXXX", "XC")
    roman.sub!("XXXX", "XL")
    roman.sub!("VIIII", "IX")
    roman.sub!("IIII", "IV")

    return roman
end
```
