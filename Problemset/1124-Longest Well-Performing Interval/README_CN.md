# 1124. 表现良好的最长时间段
给你一份工作时间表 `hours`，上面记录着某一位员工每天的工作小时数。

我们认为当员工一天中的工作小时数大于 `8` 小时的时候，那么这一天就是「**劳累的一天**」。

所谓「表现良好的时间段」，意味在这段时间内，「劳累的天数」是严格 **大于**「不劳累的天数」。

请你返回「表现良好时间段」的最大长度。

#### 示例 1:
<pre>
<strong>输入:</strong> hours = [9,9,6,0,6,6,9]
<strong>输出:</strong> 3
<strong>解释:</strong> 最长的表现良好时间段是 [9,9,6]。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> hours = [6,6,6]
<strong>输出:</strong> 0
</pre>

#### 提示:
* <code>1 <= hours.length <= 10<sup>4</sup></code>
* `0 <= hours[i] <= 16`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut diff = 0;
        let mut first_diff_index = HashMap::new();
        let mut ret = 0;

        for i in 0..hours.len() {
            diff += (hours[i] / 9) * 2 - 1;

            if diff > 0 {
                ret = i + 1;
            } else {
                if let Some(&j) = first_diff_index.get(&(diff - 1)) {
                    ret = ret.max(i - j);
                }
                if diff < 0 && !first_diff_index.contains_key(&diff) {
                    first_diff_index.insert(diff, i);
                }
            }
        }

        ret as i32
    }
}
```
