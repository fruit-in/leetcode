# 43. 字符串相乘
给定两个以字符串形式表示的非负整数 `num1` 和 `num2`，返回 `num1` 和 `num2` 的乘积，它们的乘积也表示为字符串形式。

**注意：**不能使用任何内置的 BigInteger 库或直接将输入转换为整数。

#### 示例 1:
<pre>
<strong>输入:</strong> num1 = "2", num2 = "3"
<strong>输出:</strong> "6"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> num1 = "123", num2 = "456"
<strong>输出:</strong> "56088"
</pre>

#### 提示:
* `1 <= num1.length, num2.length <= 200`
* `num1` 和 `num2` 只能由数字组成。
* `num1` 和 `num2` 都不包含任何前导零，除了数字0本身。

## 题解 (Rust)

### 1. 题解
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
