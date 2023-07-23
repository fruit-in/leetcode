# 717. 1-bit and 2-bit Characters
We have two special characters. The first character can be represented by one bit ```0```. The second character can be represented by two bits (```10``` or ```11```).

Now given a string represented by several bits. Return whether the last character must be a one-bit character or not. The given string will always end with a zero.

#### Example 1:
<pre>
<strong>Input:</strong>
bits = [1, 0, 0]
<strong>Output:</strong> True
<strong>Explanation:</strong>
The only way to decode it is two-bit character and one-bit character.
So the last character is one-bit character.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong>
bits = [1, 1, 1, 0]
<strong>Output:</strong> False
<strong>Explanation:</strong>
The only way to decode it is two-bit character and two-bit character.
So the last character is NOT one-bit character.
</pre>

#### Note:
* ```1 <= len(bits) <= 1000```.
* ```bits[i]``` is always ```0``` or ```1```.

## Solutions (Rust)

### 1. Greedy
```Rust
impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut i = bits.len() - 1;
        while i > 0 && bits[i - 1] == 1 {
            i -= 1;
        }

        (bits.len() - 1 - i) % 2 == 0
    }
}
```

### 2. Increment Pointer
```Rust
impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut i = 0;
        while i < bits.len() - 1 {
            i += 1 + bits[i] as usize;
        }
 
        i == bits.len() - 1
    }
}
```
