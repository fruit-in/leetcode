# 1344. Angle Between Hands of a Clock
Given two numbers, ```hour``` and ```minutes```. Return the smaller angle (in degrees) formed between the ```hour``` and the ```minute``` hand.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/12/26/sample_1_1673.png)
<pre>
<strong>Input:</strong> hour = 12, minutes = 30
<strong>Output:</strong> 165
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2019/12/26/sample_2_1673.png)
<pre>
<strong>Input:</strong> hour = 3, minutes = 30
<strong>Output:</strong> 75
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2019/12/26/sample_3_1673.png)
<pre>
<strong>Input:</strong> hour = 3, minutes = 15
<strong>Output:</strong> 7.5
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> hour = 4, minutes = 50
<strong>Output:</strong> 155
</pre>

#### Example 5:
<pre>
<strong>Input:</strong> hour = 12, minutes = 0
<strong>Output:</strong> 0
</pre>

#### Constraints:
* ```1 <= hour <= 12```
* ```0 <= minutes <= 59```
* Answers within ```10^-5``` of the actual value will be accepted as correct.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let m_angle = minutes as f64 * 6.;
        let h_angle = (hour % 12 * 60 + minutes) as f64 * 0.5;
        let min = m_angle.min(h_angle);
        let max = m_angle.max(h_angle);

        (max - min).min(360. - max + min)
    }
}
```
