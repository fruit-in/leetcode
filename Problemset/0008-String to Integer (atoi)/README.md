# 8. String to Integer (atoi)
Implement ```atoi``` which converts a string to an integer.

The function first discards as many whitespace characters as necessary until the first non-whitespace character is found. Then, starting from this character, takes an optional initial plus or minus sign followed by as many numerical digits as possible, and interprets them as a numerical value.

The string can contain additional characters after those that form the integral number, which are ignored and have no effect on the behavior of this function.

If the first sequence of non-whitespace characters in str is not a valid integral number, or if no such sequence exists because either str is empty or it contains only whitespace characters, no conversion is performed.

If no valid conversion could be performed, a zero value is returned.

#### Note:
* Only the space character ```' '``` is considered as whitespace character.
* Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−2<sup>31</sup>,  2<sup>31</sup> − 1]. If the numerical value is out of the range of representable values, INT_MAX (2<sup>31</sup> − 1) or INT_MIN (−2<sup>31</sup>) is returned.

#### Example 1:
<pre>
<strong>Input:</strong> "42"
<strong>Output:</strong> 42
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "   -42"
<strong>Output:</strong> -42
<strong>Explanation:</strong> The first non-whitespace character is '-', which is the minus sign.
             Then take as many numerical digits as possible, which gets 42.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> "4193 with words"
<strong>Output:</strong> 4193
<strong>Explanation:</strong> Conversion stops at digit '3' as the next character is not a numerical digit.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> "words and 987"
<strong>Output:</strong> 0
<strong>Explanation:</strong> The first non-whitespace character is 'w', which is not a numerical 
             digit or a +/- sign. Therefore no valid conversion could be performed.
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> "-91283472332"
<strong>Output:</strong> -2147483648
<strong>Explanation:</strong> The number "-91283472332" is out of the range of a 32-bit signed integer.
             Thefore INT_MIN (−2<sup>31</sup>) is returned.
</pre>

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def myAtoi(self, str: str) -> int:
        str = str.lstrip()
        sign = -1 if str and str[0] == '-' else 0
        n = 0

        if str and (str[0] == '+' or str[0] == '-'):
            str = str[1:]

        for i in range(len(str)):
            if str[i].isdigit():
                x = ord(str[i]) - 48
                if n > 214748364 or (n == 214748364 and x + sign > 7):
                    return 2147483647 if sign == 0 else -2147483648
                n *= 10
                n += x
            else:
                break

        return n if sign == 0 else -n
```
