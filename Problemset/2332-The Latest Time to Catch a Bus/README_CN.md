# 2332. 坐上公交的最晚时间
给你一个下标从 **0** 开始长度为 `n` 的整数数组 `buses` ，其中 `buses[i]` 表示第 `i` 辆公交车的出发时间。同时给你一个下标从 **0** 开始长度为 `m` 的整数数组 `passengers` ，其中 `passengers[j]` 表示第 `j` 位乘客的到达时间。所有公交车出发的时间互不相同，所有乘客到达的时间也互不相同。

给你一个整数 `capacity` ，表示每辆公交车 **最多** 能容纳的乘客数目。

每位乘客都会排队搭乘下一辆有座位的公交车。如果你在 `y` 时刻到达，公交在 `x` 时刻出发，满足 `y <= x`  且公交没有满，那么你可以搭乘这一辆公交。**最早** 到达的乘客优先上车。

返回你可以搭乘公交车的最晚到达公交站时间。你 **不能** 跟别的乘客同时刻到达。

**注意：**数组 `buses` 和 `passengers` 不一定是有序的。

#### 示例 1:
<pre>
<strong>输入:</strong> buses = [10,20], passengers = [2,17,18,19], capacity = 2
<strong>输出:</strong> 16
<strong>解释:</strong>
第 1 辆公交车载着第 1 位乘客。
第 2 辆公交车载着你和第 2 位乘客。
注意你不能跟其他乘客同一时间到达，所以你必须在第二位乘客之前到达。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> buses = [20,30,10], passengers = [19,13,26,4,25,11,21], capacity = 2
<strong>输出:</strong> 20
<strong>解释:</strong>
第 1 辆公交车载着第 4 位乘客。
第 2 辆公交车载着第 6 位和第 2 位乘客。
第 3 辆公交车载着第 1 位乘客和你。
</pre>

#### 提示:
* `n == buses.length`
* `m == passengers.length`
* <code>1 <= n, m, capacity <= 10<sup>5</sup></code>
* <code>2 <= buses[i], passengers[i] <= 10<sup>9</sup></code>
* `buses` 中的元素 **互不相同** 。
* `passengers` 中的元素 **互不相同** 。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn latest_time_catch_the_bus(
        mut buses: Vec<i32>,
        mut passengers: Vec<i32>,
        capacity: i32,
    ) -> i32 {
        let passengers_set = passengers.clone().into_iter().collect::<HashSet<_>>();
        let mut i = 0;
        let mut ret = 1;

        buses.sort_unstable();
        passengers.sort_unstable();

        for j in 0..buses.len() {
            let mut count = 0;
            let mut time = buses[j];

            while i < passengers.len() && passengers[i] <= buses[j] && count < capacity {
                count += 1;
                i += 1;
            }

            if count == capacity {
                time = passengers[i - 1];
            }

            while passengers_set.contains(&time) {
                time -= 1;
            }

            ret = ret.max(time);
        }

        ret
    }
}
```
