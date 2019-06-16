# 338. Counting Bits
Given a non negative integer number **num**. For every numbers **i** in the range **0 ≤ i ≤ num** calculate the number of 1's in their binary representation and return them as an array.

#### Example 1:
<pre>
<strong>Input:</strong> 2
<strong>Output:</strong> [0,1,1]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 5
<strong>Output:</strong> [0,1,1,2,1,2]
</pre>

#### Follow up:
* It is very easy to come up with a solution with run time <strong>O(n*sizeof(integer))</strong>. But can you do it in linear time **O(n)** /possibly in a single pass?
* Space complexity should be **O(n)**.
* Can you do it like a boss? Do it without using any builtin function like <strong>__builtin_popcount</strong> in c++ or in any other language.

## Solutions

### 1. i % n (n = 2<sup>⌊log<sub>2</sub>(n)⌋</sup>) (Rust)
```Rust
impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut n = 1;
        let mut result = vec![0];
        for i in 1..=num as usize{
            n *= (i / n);
            result.push(&result[i % n] + 1);
        }
        result
    }
}
```

### 2. i & (i - 1) (Rust)
```Rust
impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let mut result = vec![0];
        for i in 1..=num as usize{
            result.push(&result[i & (i - 1)] + 1);
        }
        result
    }
}
```
