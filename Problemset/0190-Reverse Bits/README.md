# 190. Reverse Bits
Reverse bits of a given 32 bits unsigned integer.

#### Example 1:
<pre>
<strong>Input:</strong> 00000010100101000001111010011100
<strong>Output:</strong> 00111001011110000010100101000000
<strong>Explanation:</strong> The input binary string <strong>00000010100101000001111010011100</strong> represents the unsigned integer 43261596,
so return 964176192 which its binary representation is <strong>00111001011110000010100101000000</strong>
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 11111111111111111111111111111101
<strong>Output:</strong> 10111111111111111111111111111111
<strong>Explanation:</strong> The input binary string <strong>11111111111111111111111111111101</strong> represents the unsigned integer 4294967293,
so return 3221225471 which its binary representation is <strong>10101111110010110010011101101001</strong>
</pre>

#### Note:
* Note that in some languages such as Java, there is no unsigned integer type. In this case, both input and output will be given as signed integer type and should not affect your implementation, as the internal binary representation of the integer is the same whether it is signed or unsigned.
* In Java, the compiler represents the signed integers using [2's complement notation](https://en.wikipedia.org/wiki/Two%27s_complement). Therefore, in **Example 2** above the input represents the signed integer ```-3``` and the output represents the signed integer ```-1073741825```.

#### Follow up:
If this function is called many times, how would you optimize it?

## Solutions (Python)

### 1. Straight Forward
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

### 2. Two Pointers
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

### 3. Divide & Conquer
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
