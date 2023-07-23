# 166. 分数到小数
给定两个整数，分别表示分数的分子 `numerator` 和分母 `denominator`，以 **字符串形式返回小数** 。

如果小数部分为循环小数，则将循环的部分括在括号内。

如果存在多个答案，只需返回 **任意一个** 。

对于所有给定的输入，**保证** 答案字符串的长度小于 <code>10<sup>4</sup></code> 。

#### 示例 1:
<pre>
<strong>输入:</strong> numerator = 1, denominator = 2
<strong>输出:</strong> "0.5"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> numerator = 2, denominator = 1
<strong>输出:</strong> "2"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> numerator = 4, denominator = 333
<strong>输出:</strong> "0.(012)"
</pre>

#### 提示:
* <code>-2<sup>31</sup> <= numerator, denominator <= 2<sup>31</sup> - 1</code>
* `denominator != 0`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let negative = numerator != 0 && ((numerator < 0) ^ (denominator < 0));
        let mut numerator = (numerator as i64).abs();
        let denominator = (denominator as i64).abs();
        let mut numerators = HashMap::new();
        let mut fraction = vec![];

        if negative {
            fraction.push(b'-');
        }
        fraction.append(&mut (numerator / denominator).to_string().into_bytes());
        if numerator % denominator != 0 {
            fraction.push(b'.');
        }
        numerator = (numerator % denominator) * 10;

        while numerator > 0 {
            if let Some(&i) = numerators.get(&numerator) {
                fraction.insert(i, b'(');
                fraction.push(b')');
                break;
            } else {
                fraction.push((numerator / denominator) as u8 + b'0');
                numerators.insert(numerator, fraction.len() - 1);
                numerator = (numerator % denominator) * 10;
            }
        }

        String::from_utf8(fraction).unwrap()
    }
}
```
