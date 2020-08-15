# 12. Integer to Roman
Roman numerals are represented by seven different symbols: ```I```, ```V```, ```X```, ```L```, ```C```, ```D``` and ```M```.

<pre>
<strong>Symbol</strong>       <strong>Value</strong>
I             1
V             5
X             10
L             50
C             100
D             500
M             1000
</pre>

For example, two is written as ```II``` in Roman numeral, just two one's added together. Twelve is written as, ```XII```, which is simply ```X``` + ```II```. The number twenty seven is written as ```XXVII```, which is ```XX``` + ```V``` + ```II```.

Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not ```IIII```. Instead, the number four is written as ```IV```. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as ```IX```. There are six instances where subtraction is used:
* ```I``` can be placed before ```V``` (5) and ```X``` (10) to make 4 and 9.
* ```X``` can be placed before ```L``` (50) and ```C``` (100) to make 40 and 90.
* ```C``` can be placed before ```D``` (500) and ```M``` (1000) to make 400 and 900.

Given an integer, convert it to a roman numeral. Input is guaranteed to be within the range from 1 to 3999.

#### Example 1:
<pre>
<strong>Input:</strong> 3
<strong>Output:</strong> "III"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 4
<strong>Output:</strong> "IV"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> 9
<strong>Output:</strong> "IX"
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> 58
<strong>Output:</strong> "LVIII"
<strong>Explanation:</strong> L = 50, V = 5, III = 3.
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> 1994
<strong>Output:</strong> "MCMXCIV"
<strong>Explanation:</strong> M = 1000, CM = 900, XC = 90 and IV = 4.
</pre>

## Solutions (Python)

### 1. Solution
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

## Solutions (Ruby)

### 1. Solution
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
