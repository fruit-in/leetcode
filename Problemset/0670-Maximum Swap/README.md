# 670. Maximum Swap
Given a non-negative integer, you could swap two digits **at most** once to get the maximum valued number. Return the maximum valued number you could get.

#### Example 1:
<pre>
<strong>Input:</strong> 2736
<strong>Output:</strong> 7236
<strong>Explanation:</strong> Swap the number 2 and the number 7.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> 9973
<strong>Output:</strong> 9973
<strong>Explanation:</strong> No swap.
</pre>

#### Note:
1. The given number is in the range [0, 10<sup>8</sup>]

## Solutions (Rust)

### 1. Greedy
```Rust
impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut digits = num.to_string().into_bytes();
        let mut last = [0; 10];

        for i in 0..digits.len() {
            last[(digits[i] - b'0') as usize] = i;
        }

        for i in 0..digits.len() {
            for j in (((digits[i] - b'0') as usize + 1)..=9).rev() {
                if last[j] > i {
                    digits.swap(i, last[j]);
                    return String::from_utf8(digits).unwrap().parse().unwrap();
                }
            }
        }

        num
    }
}
```
