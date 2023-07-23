# 1318. Minimum Flips to Make a OR b Equal to c
Given 3 positives numbers ```a```, ```b``` and ```c```. Return the minimum flips required in some bits of ```a``` and ```b``` to make ( ```a``` OR ```b``` == ```c``` ). (bitwise OR operation).

Flip operation consists of change **any** single bit 1 to 0 or change the bit 0 to 1 in their binary representation.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/01/06/sample_3_1676.png)
<pre>
<strong>Input:</strong> a = 2, b = 6, c = 5
<strong>Output:</strong> 3
<strong>Explanation:</strong> After flips a = 1 , b = 4 , c = 5 such that (a OR b == c)
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> a = 4, b = 2, c = 7
<strong>Output:</strong> 1
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> a = 1, b = 2, c = 3
<strong>Output:</strong> 0
</pre>

#### Constraints:
* ```1 <= a <= 10^9```
* ```1 <= b <= 10^9```
* ```1 <= c <= 10^9```

## Solutions (Rust)

### 1. Bitwise Operator
```Rust
impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut a = a;
        let mut b = b;
        let mut c = c;
        let mut ret = 0;

        while a > 0 || b > 0 || c > 0 {
            let d = a & 1;
            let e = b & 1;
            let f = c & 1;
            match (d << 2) + (e << 1) + f {
                0b001 | 0b010 | 0b100 => ret += 1,
                0b110 => ret += 2,
                _ => (),
            }
            a >>= 1;
            b >>= 1;
            c >>= 1;
        }

        ret
    }
}
```
