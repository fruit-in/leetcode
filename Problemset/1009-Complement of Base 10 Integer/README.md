# 1009. Complement of Base 10 Integer
Every non-negative integer ```N``` has a binary representation.  For example, ```5``` can be represented as ```"101"``` in binary, ```11``` as ```"1011"``` in binary, and so on.  Note that except for ```N = 0```, there are no leading zeroes in any binary representation.

The complement of a binary representation is the number in binary you get when changing every ```1``` to a ```0``` and ```0``` to a ```1```.  For example, the complement of ```"101"``` in binary is ```"010"``` in binary.

For a given number ```N``` in base-10, return the complement of it's binary representation as a base-10 integer.

#### Example 1:
<pre>
<strong>Input:</strong> 5
<strong>Output:</strong> 2
<strong>Explanation:</strong> 5 is "101" in binary, with complement "010" in binary, which is 2 in base-10.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 7
<strong>Output:</strong> 0
<strong>Explanation:</strong> 7 is "111" in binary, with complement "000" in binary, which is 0 in base-10.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> 10
<strong>Output:</strong> 5
<strong>Explanation:</strong> 10 is "1010" in binary, with complement "0101" in binary, which is 5 in base-10.
</pre>

#### Note:
1. ```0 <= N < 10^9```

## Solutions (Rust)

### 1. Mathematical
```Rust
impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        match n {
            0 => 1,
            _ => 2_i32.pow((n as f64).log2() as u32 + 1) - 1 - n,
        }
    }
}
```

### 2. Bitwise Operator
```Rust
impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        let mut ret = 0;

        for i in 0..31 {
            if n >> i == 0 && i > 0 {
                break;
            }
            if n & (1 << i) == 0 {
                ret ^= 1 << i;
            }
        }

        ret
    }
}
```
