# 2594. 修车的最少时间
给你一个整数数组 `ranks` ，表示一些机械工的 **能力值** 。<code>ranks<sub>i</sub></code> 是第 `i` 位机械工的能力值。能力值为 `r` 的机械工可以在 <code>r * n<sup>2</sup></code> 分钟内修好 `n` 辆车。

同时给你一个整数 `cars` ，表示总共需要修理的汽车数目。

请你返回修理所有汽车 **最少** 需要多少时间。

**注意：**所有机械工可以同时修理汽车。

#### 示例 1:
<pre>
<strong>输入:</strong> ranks = [4,2,3,1], cars = 10
<strong>输出:</strong> 16
<strong>解释:</strong>
- 第一位机械工修 2 辆车，需要 4 * 2 * 2 = 16 分钟。
- 第二位机械工修 2 辆车，需要 2 * 2 * 2 = 8 分钟。
- 第三位机械工修 2 辆车，需要 3 * 2 * 2 = 12 分钟。
- 第四位机械工修 4 辆车，需要 1 * 4 * 4 = 16 分钟。
16 分钟是修理完所有车需要的最少时间。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> ranks = [5,1,8], cars = 6
<strong>输出:</strong> 16
<strong>解释:</strong>
- 第一位机械工修 1 辆车，需要 5 * 1 * 1 = 5 分钟。
- 第二位机械工修 4 辆车，需要 1 * 4 * 4 = 16 分钟。
- 第三位机械工修 1 辆车，需要 8 * 1 * 1 = 8 分钟。
16 分钟时修理完所有车需要的最少时间。
</pre>

#### 提示:
* <code>1 <= ranks.length <= 10<sup>5</sup></code>
* `1 <= ranks[i] <= 100`
* <code>1 <= cars <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let mut heap = ranks
            .iter()
            .map(|&r| (-r as i64, r as i64, 0))
            .collect::<BinaryHeap<_>>();

        for _ in 0..cars {
            let mut time_rank_cars = heap.peek_mut().unwrap();
            time_rank_cars.2 += 1;
            time_rank_cars.0 = -time_rank_cars.1 * (time_rank_cars.2 + 1) * (time_rank_cars.2 + 1);
        }

        heap.iter().map(|&(t, r, n)| r * n * n).max().unwrap()
    }
}
```
