# 2188. 完成比赛的最少时间
给你一个下标从 **0** 开始的二维整数数组 `tires` ，其中 <code>tires[i] = [f<sub>i</sub>, r<sub>i</sub>]</code> 表示第 `i` 种轮胎如果连续使用，第 `x` 圈需要耗时 <code>f<sub>i</sub> * r<sub>i</sub><sup>(x-1)</sup></code> 秒。

* 比方说，如果 <code>f<sub>i</sub> = 3</code> 且 <code>r<sub>i</sub> = 2</code> ，且一直使用这种类型的同一条轮胎，那么该轮胎完成第 `1` 圈赛道耗时 `3` 秒，完成第 `2` 圈耗时 `3 * 2 = 6` 秒，完成第 `3` 圈耗时 <code>3 * 2<sup>2</sup> = 12</code> 秒，依次类推。

同时给你一个整数 `changeTime` 和一个整数 `numLaps` 。

比赛总共包含 `numLaps` 圈，你可以选择 **任意** 一种轮胎开始比赛。每一种轮胎都有 **无数条** 。每一圈后，你可以选择耗费 `changeTime` 秒 **换成** 任意一种轮胎（也可以换成当前种类的新轮胎）。

请你返回完成比赛需要耗费的 **最少** 时间。

#### 示例 1:
<pre>
<strong>输入:</strong> tires = [[2,3],[3,4]], changeTime = 5, numLaps = 4
<strong>输出:</strong> 21
<strong>解释:</strong>
第 1 圈：使用轮胎 0 ，耗时 2 秒。
第 2 圈：继续使用轮胎 0 ，耗时 2 * 3 = 6 秒。
第 3 圈：耗费 5 秒换一条新的轮胎 0 ，然后耗时 2 秒完成这一圈。
第 4 圈：继续使用轮胎 0 ，耗时 2 * 3 = 6 秒。
总耗时 = 2 + 6 + 5 + 2 + 6 = 21 秒。
完成比赛的最少时间为 21 秒。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> tires = [[1,10],[2,2],[3,4]], changeTime = 6, numLaps = 5
<strong>输出:</strong> 25
<strong>解释:</strong>
第 1 圈：使用轮胎 1 ，耗时 2 秒。
第 2 圈：继续使用轮胎 1 ，耗时 2 * 2 = 4 秒。
第 3 圈：耗时 6 秒换一条新的轮胎 1 ，然后耗时 2 秒完成这一圈。
第 4 圈：继续使用轮胎 1 ，耗时 2 * 2 = 4 秒。
第 5 圈：耗时 6 秒换成轮胎 0 ，然后耗时 1 秒完成这一圈。
总耗时 = 2 + 4 + 6 + 2 + 4 + 6 + 1 = 25 秒。
完成比赛的最少时间为 25 秒。
</pre>

#### 提示:
* <code>1 <= tires.length <= 10<sup>5</sup></code>
* `tires[i].length == 2`
* <code>1 <= f<sub>i</sub>, changeTime <= 10<sup>5</sup></code>
* <code>2 <= r<sub>i</sub> <= 105</sup></code>
* `1 <= numLaps <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_finish_time(tires: Vec<Vec<i32>>, change_time: i32, num_laps: i32) -> i32 {
        let mut laps_min = vec![i32::MAX; 18.min(num_laps as usize + 1)];
        let mut dp = vec![i32::MAX; num_laps as usize + 1];
        dp[0] = -change_time;

        for i in 0..tires.len() {
            let (f, r) = (tires[i][0], tires[i][1]);
            let mut total_time = change_time;
            let mut lap_time = f;

            for x in 1..laps_min.len() {
                total_time += lap_time;
                laps_min[x] = laps_min[x].min(total_time);
                lap_time = lap_time.saturating_mul(r);
                if lap_time >= change_time + f {
                    break;
                }
            }
        }

        for i in 0..dp.len() {
            for x in 1..laps_min.len() {
                if i + x < dp.len() {
                    dp[i + x] = dp[i + x].min(dp[i].saturating_add(laps_min[x]));
                }
            }
        }

        *dp.last().unwrap()
    }
}
```
