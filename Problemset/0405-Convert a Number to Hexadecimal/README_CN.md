# 405. 数字转换为十六进制数
给定一个整数，编写一个算法将这个数转换为十六进制数。对于负整数，我们通常使用 [补码运算](https://baike.baidu.com/item/%E8%A1%A5%E7%A0%81/6854613?fr=aladdin) 方法。

#### 注意:
1. 十六进制中所有字母(```a-f```)都必须是小写。
2. 十六进制字符串中不能包含多余的前导零。如果要转化的数为0，那么以单个字符```'0'```来表示；对于其他情况，十六进制字符串中的第一个字符将不会是0字符。 
3. 给定的数确保在32位有符号整数范围内。
4. **不能使用任何由库提供的将数字直接转换或格式化为十六进制的方法。**

#### 示例 1:
```
输入:
26

输出:
"1a"
```

#### 示例 2:
```
输入:
-1

输出:
"ffffffff"
```

## 题解 (Python)

### 1. 补码运算
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

### 2. 转换为无符号整数
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

### 3. 位操作
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
