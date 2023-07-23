# 2525. 根据规则将箱子分类
给你四个整数 `length` ，`width` ，`height` 和 `mass` ，分别表示一个箱子的三个维度和质量，请你返回一个表示箱子 **类别** 的字符串。
* 如果满足以下条件，那么箱子是 `"Bulky"` 的：
    * 箱子 **至少有一个** 维度大于等于 104 。
    * 或者箱子的 **体积** 大于等于 109 。
* 如果箱子的质量大于等于 `100` ，那么箱子是 `"Heavy"` 的。
* 如果箱子同时是 `"Bulky"` 和 `"Heavy"` ，那么返回类别为 `"Both"` 。
* 如果箱子既不是 `"Bulky"` ，也不是 `"Heavy"` ，那么返回类别为 `"Neither"` 。
* 如果箱子是 `"Bulky"` 但不是 `"Heavy"` ，那么返回类别为 `"Bulky"` 。
* 如果箱子是 `"Heavy"` 但不是 `"Bulky"` ，那么返回类别为 `"Heavy"` 。

**注意**，箱子的体积等于箱子的长度、宽度和高度的乘积。

#### 示例 1:
<pre>
<strong>输入:</strong> length = 1000, width = 35, height = 700, mass = 300
<strong>输出:</strong> "Heavy"
<strong>解释:</strong>
箱子没有任何维度大于等于 10<sup>4</sup> 。
体积为 24500000 <= 10<sup>9</sup> 。所以不能归类为 "Bulky" 。
但是质量 >= 100 ，所以箱子是 "Heavy" 的。
由于箱子不是 "Bulky" 但是是 "Heavy" ，所以我们返回 "Heavy" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> length = 200, width = 50, height = 800, mass = 50
<strong>输出:</strong> "Neither"
<strong>解释:</strong>
箱子没有任何维度大于等于 10<sup>4</sup> 。
体积为 8 * 10<sup>6</sup> <= 10<sup>9</sup> 。所以不能归类为 "Bulky" 。
质量小于 100 ，所以不能归类为 "Heavy" 。
由于不属于上述两者任何一类，所以我们返回 "Neither" 。
</pre>

#### 提示:
* <code>1 <= length, width, height <= 10<sup>5</sup></code>
* <code>1 <= mass <= 10<sup>3</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        let volume = length as i64 * width as i64 * height as i64;
        let is_bulky = length.max(width).max(height) >= 10000 || volume >= 1_000_000_000;
        let is_heavy = mass >= 100;

        match (is_bulky, is_heavy) {
            (true, true) => "Both",
            (false, false) => "Neither",
            (true, false) => "Bulky",
            (false, true) => "Heavy",
        }
        .to_string()
    }
}
```
