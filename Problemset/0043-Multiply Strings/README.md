# 43. Multiply Strings
Given two non-negative integers `num1` and `num2` represented as strings, return the product of `num1` and `num2`, also represented as a string.

**Note:** You must not use any built-in BigInteger library or convert the inputs to integer directly.

#### Example 1:
<pre>
<strong>Input:</strong> num1 = "2", num2 = "3"
<strong>Output:</strong> "6"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> num1 = "123", num2 = "456"
<strong>Output:</strong> "56088"
</pre>

#### Constraints:
* `1 <= num1.length, num2.length <= 200`
* `num1` and `num2` consist of digits only.
* Both `num1` and `num2` do not contain any leading zero, except the number `0` itself.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut num1 = num1.bytes().map(|x| x - b'0').rev().collect::<Vec<_>>();
        let mut num2 = num2.bytes().map(|x| x - b'0').rev().collect::<Vec<_>>();
        let mut product = vec![0; num1.len() + num2.len()];

        for i in 0..num1.len() {
            for j in 0..num2.len() {
                product[i + j] += (num1[i] * num2[j]) as u32;
            }
        }

        for i in 0..product.len() {
            if product[i] > 9 {
                product[i + 1] += product[i] / 10;
                product[i] %= 10;
            }
        }

        while product.len() > 1 && *product.last().unwrap() == 0 {
            product.pop();
        }

        product
            .into_iter()
            .rev()
            .map(|x| char::from_digit(x, 10).unwrap())
            .collect()
    }
}
```
