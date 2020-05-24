# 539. 最小时间差
给定一个 24 小时制（小时:分钟）的时间列表，找出列表中任意两个时间的最小时间差并以分钟数表示。

#### 示例 1:
<pre>
<strong>输入:</strong> ["23:59","00:00"]
<strong>输出:</strong> 1
</pre>

#### 备注:
1. 列表中时间数在 2~20000 之间。
2. 每个时间取值在 00:00~23:59 之间。

## 题解 (Rust)

### 1. 排序
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
