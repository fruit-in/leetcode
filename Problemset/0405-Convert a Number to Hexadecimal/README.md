# 405. Convert a Number to Hexadecimal
Given an integer, write an algorithm to convert it to hexadecimal. For negative integer, [two’s complement](https://en.wikipedia.org/wiki/Two%27s_complement) method is used.

#### Note:
1. All letters in hexadecimal (```a-f```) must be in lowercase.
2. The hexadecimal string must not contain extra leading ```0```s. If the number is zero, it is represented by a single zero character ```'0'```; otherwise, the first character in the hexadecimal string will not be the zero character.
3. The given number is guaranteed to fit within the range of a 32-bit signed integer.
4. You **must not use *any* method provided by the library** which converts/formats the number to hex directly.

#### Example 1:
```
Input:
26

Output:
"1a"
```

#### Example 2:
```
Input:
-1

Output:
"ffffffff"
```

## Solutions (Python)

### 1. Two's Complement
```Python3
class Solution:
    def toHex(self, num: int) -> str:
        hex_ch = "0123456789abcdef"
        hex_n = ""

        if num == 0:
            return "0"
        elif num > 0:
            while num > 0:
                hex_n += hex_ch[num % 16]
                num //= 16
        elif num < 0:
            num = -num
            plus_one = True

            while num > 0:
                if plus_one and num % 16 != 0:
                    hex_n += hex_ch[15 - num % 16 + 1]
                    plus_one = False
                elif plus_one:
                    hex_n += '0'
                else:
                    hex_n += hex_ch[15 - num % 16]
                num //= 16

            hex_n += 'f' * (8 - len(hex_n))

        return hex_n[::-1]
```

### 2. Convert to Unsigned Integer
```Python3
class Solution:
    def toHex(self, num: int) -> str:
        hex_ch = "0123456789abcdef"
        hex_n = ""

        if num == 0:
            return "0"
        elif num < 0:
            num = 4294967296 + num

        while num > 0:
            hex_n += hex_ch[num % 16]
            num //= 16

        return hex_n[::-1]
```

### 3. Bitwise Operator
```Python3
class Solution:
    def toHex(self, num: int) -> str:
        hex_ch = "0123456789abcdef"
        hex_n = ""

        if num == 0:
            return "0"

        if num < 0:
            num &= 0x7fffffff
            hex_n += hex_ch[num & 0xf]
            num >>= 4
            num |= 0x08000000

        while num != 0:
            hex_n += hex_ch[num & 0xf]
            num >>= 4

        return hex_n[::-1]
```
