# 2525. Categorize Box According to Criteria
Given four integers `length`, `width`, `height`, and `mass`, representing the dimensions and mass of a box, respectively, return *a string representing the **category** of the box*.
* The box is `"Bulky"` if:
    * **Any** of the dimensions of the box is greater or equal to <code>10<sup>4</sup></code>.
    * Or, the **volume** of the box is greater or equal to <code>10<sup>9</sup></code>.
* If the mass of the box is greater or equal to `100`, it is `"Heavy"`.
* If the box is both `"Bulky"` and `"Heavy"`, then its category is `"Both"`.
* If the box is neither `"Bulky"` nor `"Heavy"`, then its category is `"Neither"`.
* If the box is `"Bulky"` but not `"Heavy"`, then its category is `"Bulky"`.
* If the box is `"Heavy"` but not `"Bulky"`, then its category is `"Heavy"`.

**Note** that the volume of the box is the product of its length, width and height.

#### Example 1:
<pre>
<strong>Input:</strong> length = 1000, width = 35, height = 700, mass = 300
<strong>Output:</strong> "Heavy"
<strong>Explanation:</strong>
None of the dimensions of the box is greater or equal to 10<sup>4</sup>.
Its volume = 24500000 <= 10<sup>9</sup>. So it cannot be categorized as "Bulky".
However mass >= 100, so the box is "Heavy".
Since the box is not "Bulky" but "Heavy", we return "Heavy".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> length = 200, width = 50, height = 800, mass = 50
<strong>Output:</strong> "Neither"
<strong>Explanation:</strong>
None of the dimensions of the box is greater or equal to 10<sup>4</sup>.
Its volume = 8 * 10<sup>6</sup> <= 10<sup>9</sup>. So it cannot be categorized as "Bulky".
Its mass is also less than 100, so it cannot be categorized as "Heavy" either.
Since its neither of the two above categories, we return "Neither".
</pre>

#### Constraints:
* <code>1 <= length, width, height <= 10<sup>5</sup></code>
* <code>1 <= mass <= 10<sup>3</sup></code>

## Solutions (Rust)

### 1. Solution
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
