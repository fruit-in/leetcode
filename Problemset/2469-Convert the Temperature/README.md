# 2469. Convert the Temperature
You are given a non-negative floating point number rounded to two decimal places `celsius`, that denotes the **temperature in Celsius**.

You should convert Celsius into **Kelvin** and **Fahrenheit** and return it as an array `ans = [kelvin, fahrenheit]`.

Return *the array* `ans`. Answers within <code>10<sup>-5</sup></code> of the actual answer will be accepted.

**Note that**:
* `Kelvin = Celsius + 273.15`
* `Fahrenheit = Celsius * 1.80 + 32.00`

#### Example 1:
<pre>
<strong>Input:</strong> celsius = 36.50
<strong>Output:</strong> [309.65000,97.70000]
<strong>Explanation:</strong> Temperature at 36.50 Celsius converted in Kelvin is 309.65 and converted in Fahrenheit is 97.70.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> celsius = 122.11
<strong>Output:</strong> [395.26000,251.79800]
<strong>Explanation:</strong> Temperature at 122.11 Celsius converted in Kelvin is 395.26 and converted in Fahrenheit is 251.798.
</pre>

#### Constraints:
* `0 <= celsius <= 1000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        vec![celsius + 273.15, celsius * 1.8 + 32.]
    }
}
```
