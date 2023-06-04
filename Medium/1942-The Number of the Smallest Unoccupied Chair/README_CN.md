# 1942. 最小未被占据椅子的编号
有 `n` 个朋友在举办一个派对，这些朋友从 `0` 到 `n - 1` 编号。派对里有 **无数** 张椅子，编号为 `0` 到 `infinity` 。当一个朋友到达派对时，他会占据 **编号最小** 且未被占据的椅子。

* 比方说，当一个朋友到达时，如果椅子 `0` ，`1` 和 `5` 被占据了，那么他会占据 `2` 号椅子。

当一个朋友离开派对时，他的椅子会立刻变成未占据状态。如果同一时刻有另一个朋友到达，可以立即占据这张椅子。

给你一个下标从 **0** 开始的二维整数数组 `times` ，其中 <code>times[i] = [arrival<sub>i</sub>, leaving<sub>i</sub>]</code> 表示第 `i` 个朋友到达和离开的时刻，同时给你一个整数 `targetFriend` 。所有到达时间 **互不相同** 。

请你返回编号为 `targetFriend` 的朋友占据的 **椅子编号** 。

#### 示例 1:
<pre>
<strong>输入:</strong> times = [[1,4],[2,3],[4,6]], targetFriend = 1
<strong>输出:</strong> 1
<strong>解释:</strong>
- 朋友 0 时刻 1 到达，占据椅子 0 。
- 朋友 1 时刻 2 到达，占据椅子 1 。
- 朋友 1 时刻 3 离开，椅子 1 变成未占据。
- 朋友 0 时刻 4 离开，椅子 0 变成未占据。
- 朋友 2 时刻 4 到达，占据椅子 0 。
朋友 1 占据椅子 1 ，所以返回 1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> times = [[3,10],[1,5],[2,6]], targetFriend = 0
<strong>输出:</strong> 2
<strong>解释:</strong>
- 朋友 1 时刻 1 到达，占据椅子 0 。
- 朋友 2 时刻 2 到达，占据椅子 1 。
- 朋友 0 时刻 3 到达，占据椅子 2 。
- 朋友 1 时刻 5 离开，椅子 0 变成未占据。
- 朋友 2 时刻 6 离开，椅子 1 变成未占据。
- 朋友 0 时刻 10 离开，椅子 2 变成未占据。
朋友 0 占据椅子 2 ，所以返回 2 。
</pre>

#### 提示:
* `n == times.length`
* <code>2 <= n <= 10<sup>4</sup></code>
* `times[i].length == 2`
* <code>1 <= arrival<sub>i</sub> < leaving<sub>i</sub> <= 10<sup>5</sup></code>
* `0 <= targetFriend <= n - 1`
* 每个 <code>arrival<sub>i</sub></code> 时刻 **互不相同** 。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let target_arrival = times[target_friend as usize][0];
        let mut chair_inf = 0;
        let mut chair_heap = BinaryHeap::new();
        let mut leaving_heap = BinaryHeap::new();
        let mut times = times;
        times.sort_unstable();

        for time in &times {
            while -leaving_heap.peek().unwrap_or(&(-100001, 0)).0 <= time[0] {
                chair_heap.push(leaving_heap.pop().unwrap().1);
            }

            match chair_heap.pop() {
                Some(chair) if time[0] == target_arrival => return -chair,
                Some(chair) => leaving_heap.push((-time[1], chair)),
                None if time[0] == target_arrival => return chair_inf,
                None => {
                    leaving_heap.push((-time[1], -chair_inf));
                    chair_inf += 1;
                }
            }
        }

        unreachable!()
    }
}
```
