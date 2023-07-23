# 166. Fraction to Recurring Decimal
Given two integers representing the `numerator` and `denominator` of a fraction, return *the fraction in string format*.

If the fractional part is repeating, enclose the repeating part in parentheses.

If multiple answers are possible, return **any of them**.

It is **guaranteed** that the length of the answer string is less than <code>10<sup>4</sup></code> for all the given inputs.

#### Example 1:
<pre>
<strong>Input:</strong> numerator = 1, denominator = 2
<strong>Output:</strong> "0.5"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> numerator = 2, denominator = 1
<strong>Output:</strong> "2"
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> numerator = 4, denominator = 333
<strong>Output:</strong> "0.(012)"
</pre>

#### Constraints:
* <code>-2<sup>31</sup> <= numerator, denominator <= 2<sup>31</sup> - 1</code>
* `denominator != 0`

## Solutions (Rust)

### 1. Solution
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
