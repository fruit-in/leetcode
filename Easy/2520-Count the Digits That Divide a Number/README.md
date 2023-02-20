# 2520. Count the Digits That Divide a Number
Given an integer `num`, return *the number of digits in* `num` *that divide* `num`.

An integer `val` divides `nums` if `nums % val == 0`.

#### Example 1:
<pre>
<strong>Input:</strong> num = 7
<strong>Output:</strong> 1
<strong>Explanation:</strong> 7 divides itself, hence the answer is 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num = 121
<strong>Output:</strong> 2
<strong>Explanation:</strong> 121 is divisible by 1, but not 2. Since 1 occurs twice as a digit, we return 2.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> num = 1248
<strong>Output:</strong> 4
<strong>Explanation:</strong> 1248 is divisible by all of its digits, hence the answer is 4.
</pre>

#### Constraints:
* <code>1 <= num <= 10<sup>9</sup></code>
* `num` does not contain `0` as one of its digits.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut x = num;
        let mut ret = 0;

        while x > 0 {
            if num % (x % 10) == 0 {
                ret += 1;
            }
            x /= 10;
        }

        ret
    }
}
```
