# 1344. 时钟指针的夹角
给你两个数 ```hour``` 和 ```minutes``` 。请你返回在时钟上，由给定时间的时针和分针组成的较小角的角度（60 单位制）。

#### 示例 1:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/02/08/sample_1_1673.png)
<pre>
<strong>输入:</strong> hour = 12, minutes = 30
<strong>输出:</strong> 165
</pre>

#### 示例 2:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/02/08/sample_2_1673.png)
<pre>
<strong>输入:</strong> hour = 3, minutes = 30
<strong>输出:</strong> 75
</pre>

#### 示例 3:
![](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/02/08/sample_3_1673.png)
<pre>
<strong>输入:</strong> hour = 3, minutes = 15
<strong>输出:</strong> 7.5
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> hour = 4, minutes = 50
<strong>输出:</strong> 155
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> hour = 12, minutes = 0
<strong>输出:</strong> 0
</pre>

#### 提示:
* ```1 <= hour <= 12```
* ```0 <= minutes <= 59```
* 与标准答案误差在 ```10^-5``` 以内的结果都被视为正确结果。

## 题解 (Rust)

### 1. 题解
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
