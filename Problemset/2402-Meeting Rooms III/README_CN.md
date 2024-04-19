# 2402. 会议室 III
给你一个整数 `n` ，共有编号从 `0` 到 `n - 1` 的 `n` 个会议室。

给你一个二维整数数组 `meetings` ，其中 <code>meetings[i] = [start<sub>i</sub>, end<sub>i</sub>]</code> 表示一场会议将会在 **半闭** 时间区间 <code>[start<sub>i</sub>, end<sub>i</sub>)</code> 举办。所有 <code>start<sub>i</sub></code> 的值 **互不相同** 。

会议将会按以下方式分配给会议室：

1. 每场会议都会在未占用且编号 **最小** 的会议室举办。
2. 如果没有可用的会议室，会议将会延期，直到存在空闲的会议室。延期会议的持续时间和原会议持续时间 **相同** 。
3. 当会议室处于未占用状态时，将会优先提供给原 **开始** 时间更早的会议。

返回举办最多次会议的房间 **编号** 。如果存在多个房间满足此条件，则返回编号 **最小** 的房间。

**半闭区间** `[a, b)` 是 `a` 和 `b` 之间的区间，**包括** `a` 但 **不包括** `b` 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 2, meetings = [[0,10],[1,5],[2,7],[3,4]]
<strong>输出:</strong> 0
<strong>解释:</strong>
- 在时间 0 ，两个会议室都未占用，第一场会议在会议室 0 举办。
- 在时间 1 ，只有会议室 1 未占用，第二场会议在会议室 1 举办。
- 在时间 2 ，两个会议室都被占用，第三场会议延期举办。
- 在时间 3 ，两个会议室都被占用，第四场会议延期举办。
- 在时间 5 ，会议室 1 的会议结束。第三场会议在会议室 1 举办，时间周期为 [5,10) 。
- 在时间 10 ，两个会议室的会议都结束。第四场会议在会议室 0 举办，时间周期为 [10,11) 。
会议室 0 和会议室 1 都举办了 2 场会议，所以返回 0 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 3, meetings = [[1,20],[2,10],[3,5],[4,9],[6,8]]
<strong>输出:</strong> 1
<strong>解释:</strong>
- 在时间 1 ，所有三个会议室都未占用，第一场会议在会议室 0 举办。
- 在时间 2 ，会议室 1 和 2 未占用，第二场会议在会议室 1 举办。
- 在时间 3 ，只有会议室 2 未占用，第三场会议在会议室 2 举办。
- 在时间 4 ，所有三个会议室都被占用，第四场会议延期举办。
- 在时间 5 ，会议室 2 的会议结束。第四场会议在会议室 2 举办，时间周期为 [5,10) 。
- 在时间 6 ，所有三个会议室都被占用，第五场会议延期举办。
- 在时间 10 ，会议室 1 和 2 的会议结束。第五场会议在会议室 1 举办，时间周期为 [10,12) 。
会议室 1 和会议室 2 都举办了 2 场会议，所以返回 1 。
</pre>

#### 提示:
* `1 <= n <= 100`
* <code>1 <= meetings.length <= 10<sup>5</sup></code>
* `meetings[i].length == 2`
* <code>0 <= start<sub>i</sub> < end<sub>i</sub> <= 5 * 10<sup>5</sup></code>
* <code>start<sub>i</sub></code> 的所有值 **互不相同**

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut meetings = meetings;
        let mut free = (-(n - 1)..=0).collect::<BinaryHeap<_>>();
        let mut used = BinaryHeap::<(i64, i32)>::new();
        let mut time = 0_i64;
        let mut count = vec![0; n as usize];

        meetings.sort_unstable();

        for meeting in &meetings {
            let (start, end) = (meeting[0] as i64, meeting[1] as i64);

            if free.is_empty() {
                time = time.max(-used.peek().unwrap().0);
            }
            time = time.max(start);

            while let Some(&(t, r)) = used.peek() {
                if -t <= time {
                    free.push(r);
                    used.pop();
                } else {
                    break;
                }
            }

            used.push((start - end - time, *free.peek().unwrap()));
            count[(-free.pop().unwrap()) as usize] += 1;
        }

        (0..n).max_by_key(|&r| (count[r as usize], -r)).unwrap()
    }
}
```
