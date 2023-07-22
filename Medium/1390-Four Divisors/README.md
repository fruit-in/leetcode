# 1390. Four Divisors
Given an integer array `nums`, return *the sum of divisors of the integers in that array that have exactly four divisors*. If there is no such integer in the array, return `0`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [21,4,7]
<strong>Output:</strong> 32
<strong>Explanation:</strong>
21 has 4 divisors: 1, 3, 7, 21
4 has 3 divisors: 1, 2, 4
7 has 2 divisors: 1, 7
The answer is the sum of divisors of 21 only.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [21,21]
<strong>Output:</strong> 64
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,2,3,4,5]
<strong>Output:</strong> 0
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>4</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut ret = 0;

        for &x in &nums {
            let mut tmp = 0;

            for y in 2..=(x as f64).sqrt() as i32 {
                if x % y == 0 {
                    if tmp > 0 || x == y * y {
                        tmp = 0;
                        break;
                    }
                    tmp += y + x / y;
                }
            }

            if tmp > 0 {
                ret += 1 + x + tmp;
            }
        }

        ret
    }
}
```
