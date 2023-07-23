# 7. 整数反转
给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。

#### 示例 1:
<pre>
<strong>输入:</strong> 123
<strong>输出:</strong> 321
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> -123
<strong>输出:</strong> -321
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> 120
<strong>输出:</strong> 21
</pre>

#### 注意:
假设我们的环境只能存储得下 32 位的有符号整数，则其数值范围为 [−2<sup>31</sup>,  2<sup>31</sup> − 1]。请根据这个假设，如果反转后整数溢出那么就返回 0。

## 题解 (Python)

### 1. 弹出和推入数字
```Python3
class Solution:
    def reverse(self, x: int) -> int:
        result = 0
        xc = abs(x)
        while xc != 0:
            result *= 10
            result += xc % 10
            xc //= 10
        if result < -(2 ** 31) or result > (2 ** 31) - 1:
            return 0
        elif x >= 0:
            return result
        else:
            return -result
```
