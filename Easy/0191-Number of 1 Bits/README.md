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

### 1. Check Every Bits (Rust)
```Rust
impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        if n == 0 {
            0
        } else {
            Self::hamming_weight(n >> 1) + (n & 1)
        }
    }
}
```

### 2. n & (n - 1) (Rust)
```Rust
impl Solution {
    pub fn hamming_weight(n: u32) -> i32 {
        let mut ans = 0;
        let mut n = n;
        while n != 0 {
            ans += 1;
            n &= n - 1;
        }
        ans
    }
}
```
