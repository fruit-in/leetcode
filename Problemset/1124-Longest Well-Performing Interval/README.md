# 1124. Longest Well-Performing Interval
We are given `hours`, a list of the number of hours worked per day for a given employee.

A day is considered to be a *tiring day* if and only if the number of hours worked is (strictly) greater than `8`.

A *well-performing interval* is an interval of days for which the number of tiring days is strictly larger than the number of non-tiring days.

Return the length of the longest well-performing interval.

#### Example 1:
<pre>
<strong>Input:</strong> hours = [9,9,6,0,6,6,9]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The longest well-performing interval is [9,9,6].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> hours = [6,6,6]
<strong>Output:</strong> 0
</pre>

#### Constraints:
* <code>1 <= hours.length <= 10<sup>4</sup></code>
* `0 <= hours[i] <= 16`

## Solutions (Rust)

### 1. Solution
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
