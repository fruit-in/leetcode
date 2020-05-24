# 539. Minimum Time Difference
Given a list of 24-hour clock time points in "Hour:Minutes" format, find the minimum **minutes** difference between any two time points in the list.

#### Example 1:
<pre>
<strong>Input:</strong> ["23:59","00:00"]
<strong>Output:</strong> 1
</pre>

#### Note:
1. The number of time points in the given list is at least 2 and won't exceed 20000.
2. The input time is legal and ranges from 00:00 to 23:59.

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut minutes = time_points
            .iter()
            .map(|time| time.split(':').map(|n| n.parse::<i32>().unwrap()))
            .map(|mut h_m| h_m.next().unwrap() * 60 + h_m.next().unwrap())
            .collect::<Vec<i32>>();

        minutes.sort_unstable();

        let mut ret = 1440 + minutes[0] - minutes.last().unwrap();

        for i in 0..(minutes.len() - 1) {
            ret = ret.min(minutes[i + 1] - minutes[i]);
        }

        ret
    }
}
```
