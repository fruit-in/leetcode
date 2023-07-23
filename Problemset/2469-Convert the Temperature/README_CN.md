# 2469. 温度转换
给你一个四舍五入到两位小数的非负浮点数 `celsius` 来表示温度，以 **摄氏度（Celsius）**为单位。

你需要将摄氏度转换为 **开氏度（Kelvin）**和 **华氏度（Fahrenheit）**，并以数组 `ans = [kelvin, fahrenheit]` 的形式返回结果。

返回数组 *`ans`* 。与实际答案误差不超过 <code>10<sup>-5</sup></code> 的会视为正确答案。

**注意：**
* `开氏度 = 摄氏度 + 273.15`
* `华氏度 = 摄氏度 * 1.80 + 32.00`

#### 示例 1:
<pre>
<strong>输入:</strong> celsius = 36.50
<strong>输出:</strong> [309.65000,97.70000]
<strong>解释:</strong> 36.50 摄氏度：转换为开氏度是 309.65 ，转换为华氏度是 97.70 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> celsius = 122.11
<strong>输出:</strong> [395.26000,251.79800]
<strong>解释:</strong> 122.11 摄氏度：转换为开氏度是 395.26 ，转换为华氏度是 251.798 。
</pre>

#### 提示:
* `0 <= celsius <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn convert_temperature(celsius: f64) -> Vec<f64> {
        vec![celsius + 273.15, celsius * 1.8 + 32.]
    }
}
```
