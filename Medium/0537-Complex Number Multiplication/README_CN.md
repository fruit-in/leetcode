# 537. 复数乘法
给定两个表示[复数](https://baike.baidu.com/item/%E5%A4%8D%E6%95%B0/254365?fr=aladdin)的字符串。

返回表示它们乘积的字符串。注意，根据定义 i<sup>2</sup> = -1 。

#### 示例 1:
<pre>
<strong>输入:</strong> "1+1i", "1+1i"
<strong>输出:</strong> "0+2i"
<strong>解释:</strong> (1 + i) * (1 + i) = 1 + i<sup>2</sup> + 2 * i = 2i ，你需要将它转换为 0+2i 的形式。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "1+-1i", "1+-1i"
<strong>输出:</strong> "0+-2i"
<strong>解释:</strong> (1 - i) * (1 - i) = 1 + i<sup>2</sup> - 2 * i = -2i ，你需要将它转换为 0+-2i 的形式。 
</pre>

#### 注意:
1. 输入字符串不包含额外的空格。
2. 输入字符串将以 **a+bi** 的形式给出，其中整数 **a** 和 **b** 的范围均在 [-100, 100] 之间。**输出也应当符合这种形式**。

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def complexNumberMultiply(self, a: str, b: str) -> str:
        a = [int(x) for x in a[:-1].split('+')]
        b = [int(x) for x in b[:-1].split('+')]

        c = a[0] * b[0] - a[1] * b[1]
        d = a[0] * b[1] + a[1] * b[0]

        return "%d+%di" % (c, d)
```
