# 1281. Subtract the Product and Sum of Digits of an Integer
Given an integer number ```n```, return the difference between the product of its digits and the sum of its digits.

#### Example 1:
<pre>
<strong>Input:</strong> n = 234
<strong>Output:</strong> 15
<strong>Explanation:</strong>
Product of digits = 2 * 3 * 4 = 24
Sum of digits = 2 + 3 + 4 = 9
Result = 24 - 9 = 15
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 4421
<strong>Output:</strong> 21
<strong>Explanation:</strong>
Product of digits = 4 * 4 * 2 * 1 = 32
Sum of digits = 4 + 4 + 2 + 1 = 11
Result = 32 - 11 = 21
</pre>

#### Constraints:
* ```1 <= n <= 10^5```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut n = n;
        let mut digits = Vec::with_capacity(6);

        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }

        digits.iter().product::<i32>() - digits.iter().sum::<i32>()
    }
}
```
