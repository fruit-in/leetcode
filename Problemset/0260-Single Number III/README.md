# 260. Single Number III
Given an array of numbers ```nums```, in which exactly two elements appear only once and all the other elements appear exactly twice. Find the two elements that appear only once.

#### Example:
<pre>
<strong>Input:</strong> [1,2,1,3,2,5]
<strong>Output:</strong> [3,5]
</pre>

#### Note:
1. The order of the result is not important. So in the above example, ```[5, 3]``` is also correct.
2. Your algorithm should run in linear runtime complexity. Could you implement it using only constant space complexity?

## Solutions (Rust)

### 1. XOR
```Rust
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let acc = nums.iter().fold(0, |acc, n| acc ^ n);
        let mask = acc & (-acc);
        let mut ret = vec![0, 0];

        for n in nums {
            match n & mask {
                0 => ret[0] ^= n,
                _ => ret[1] ^= n,
            }
        }

        ret
    }
}
```
