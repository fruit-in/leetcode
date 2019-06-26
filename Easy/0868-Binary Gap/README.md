# 868. Binary Gap
Given a positive integer <code>N</code>, find and return the longest distance between two consecutive 1's in the binary representation of <code>N</code>.

If there aren't two consecutive 1's, return 0.

#### Example 1:
<pre>
<strong>Input:</strong> 22
<strong>Output:</strong> 2
<strong>Explanation:</strong> 
22 in binary is 0b10110.
In the binary representation of 22, there are three ones, and two consecutive pairs of 1's.
The first consecutive pair of 1's have distance 2.
The second consecutive pair of 1's have distance 1.
The answer is the largest of these two distances, which is 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 5
<strong>Output:</strong> 2
<strong>Explanation:</strong> 
5 in binary is 0b101.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> 6
<strong>Output:</strong> 1
<strong>Explanation:</strong> 
6 in binary is 0b110.
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> 8
<strong>Output:</strong> 0
<strong>Explanation:</strong> 
8 in binary is 0b1000.
There aren't any consecutive pairs of 1's in the binary representation of 8, so we return 0.
</pre>

#### Note:
* <code>1 <= N <= 10<sup>9</sup></code>

## Solutions

### 1. One Pass (Rust)
```Rust
impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut n = n;
        let mut longest = 0;
        let mut gap = 0;
        while n & 1 == 0 {
            n >>= 1;
        }
        while n != 0 {
            n >>= 1;
            gap += 1;
            if n & 1 == 1 {
                longest = longest.max(gap);
                gap = 0;
            }
        }
        longest
    }
}
```

### 2. Store Indexes (Rust)
```Rust
impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut longest = 0;
        let mut indexes = vec![0; 32];
        let mut i = 0;
        for j in 0..32 {
            if (n >> j) & 1 == 1 {
                indexes[i] = j;
                i += 1;
            }
        }
        i = 1;
        while indexes[i] > indexes[i - 1] {
            longest = longest.max(indexes[i] - indexes[i - 1]);
            i += 1;
        }
        longest
    }
}
```
