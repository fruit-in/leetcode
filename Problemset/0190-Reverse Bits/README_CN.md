# 190. 颠倒二进制位
颠倒给定的 32 位无符号整数的二进制位。

#### 示例 1:
<pre>
<strong>输入:</strong> 00000010100101000001111010011100
<strong>输出:</strong> 00111001011110000010100101000000
<strong>解释:</strong> 输入的二进制串 <strong>00000010100101000001111010011100</strong> 表示无符号整数 <strong>43261596</strong>，
      因此返回 964176192，其二进制表示形式为 <strong>00111001011110000010100101000000</strong>。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 11111111111111111111111111111101
<strong>输出:</strong> 10111111111111111111111111111111
<strong>解释:</strong> 输入的二进制串 <strong>11111111111111111111111111111101</strong> 表示无符号整数 <strong>4294967293</strong>，
      因此返回 3221225471 其二进制表示形式为 <strong>10101111110010110010011101101001</strong>。
</pre>

#### 提示:
* 请注意，在某些语言（如 Java）中，没有无符号整数类型。在这种情况下，输入和输出都将被指定为有符号整数类型，并且不应影响您的实现，因为无论整数是有符号的还是无符号的，其内部的二进制表示形式都是相同的。
* 在 Java 中，编译器使用[二进制补码](https://baike.baidu.com/item/%E8%A1%A5%E7%A0%81/6854613?fromtitle=%E4%BA%8C%E8%BF%9B%E5%88%B6%E8%A1%A5%E7%A0%81&fromid=5295284)记法来表示有符号整数。因此，在上面的 **示例 2** 中，输入表示有符号整数 ```-3```，输出表示有符号整数 ```-1073741825```。

#### 进阶:
如果多次调用这个函数，你将如何优化你的算法？

## 题解 (Python)

### 1. 直接
```Python
class Solution:
    # @param n, an integer
    # @return an integer
    def reverseBits(self, n):
        ret = 0
        for i in range(32):
            ret = (ret << 1) + (n & 1)
            n >>= 1
        return ret
```

### 2. 双指针
```Python
class Solution:
    # @param n, an integer
    # @return an integer
    def reverseBits(self, n):
        lo, hi = 1, 1 << 31
        for _ in range(16):
            if n & lo and not n & hi:
                n = (n | hi) & ~lo
            elif not n & lo and n & hi:
                n = (n | lo) & ~hi
            lo <<= 1
            hi >>= 1
        return n
```

### 3. 分治法
```Python
class Solution:
    # @param n, an integer
    # @return an integer
    def reverseBits(self, n):
        def reverse(n, lo, hi):
            if lo == hi:
                return n
            m = (lo + hi) / 2
            lo_val = n % (2 << m)
            hi_val = n - lo_val
            lo_rev = reverse(lo_val, lo , m)
            hi_rev = reverse(hi_val, m + 1, hi)
            return lo_rev * (1 << (hi - m)) + hi_rev / (1 << (hi - m))

        return reverse(n, 0, 31)
```
