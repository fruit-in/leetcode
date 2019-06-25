# 191. Number of 1 Bits
Write a function that takes an unsigned integer and return the number of '1' bits it has (also known as the Hamming weight).

#### Example 1:
<pre>
<strong>Input:</strong> 00000000000000000000000000001011
<strong>Output:</strong> 3
<strong>Explanation:</strong> The input binary string <strong>00000000000000000000000000001011</strong> has a total of three '1' bits.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 00000000000000000000000010000000
<strong>Output:</strong> 1
<strong>Explanation:</strong> The input binary string <strong>00000000000000000000000010000000</strong> has a total of one '1' bit.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> 11111111111111111111111111111101
<strong>Output:</strong> 31
<strong>Explanation:</strong> The input binary string <strong>11111111111111111111111111111101</strong> has a total of thirty one '1' bits.
</pre>

#### Note:
* Note that in some languages such as Java, there is no unsigned integer type. In this case, the input will be given as signed integer type and should not affect your implementation, as the internal binary representation of the integer is the same whether it is signed or unsigned.
* In Java, the compiler represents the signed integers using 2's complement notation. Therefore, in **Example 3** above the input represents the signed integer <code>-3</code>.

#### Follow up:
If this function is called many times, how would you optimize it?

## Solutions

### 1. Check Every Bits (C)
```C
int hammingWeight(uint32_t n){
    return n ? hammingWeight(n >> 1) + (n & 1) : 0;
}
```

### 2. n & (n - 1) (C)
```C
int hammingWeight(uint32_t n){
    int ones = 0;
    while(n != 0){
        ones++;
        n &= n - 1;
    }
    return ones;
}
```
