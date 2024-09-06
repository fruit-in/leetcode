# 2188. Minimum Time to Finish the Race
You are given a **0-indexed** 2D integer array `tires` where <code>tires[i] = [f<sub>i</sub>, r<sub>i</sub>]</code> indicates that the <code>i<sup>th</sup></code> tire can finish its <code>x<sup>th</sup></code> successive lap in <code>f<sub>i</sub> * r<sub>i</sub><sup>(x-1)</sup></code> seconds.

* For example, if <code>f<sub>i</sub> = 3</code> and <code>r<sub>i</sub> = 2</code>, then the tire would finish its <code>1<sup>st</sup></code> lap in `3` seconds, its <code>2<sup>nd</sup></code> lap in `3 * 2 = 6` seconds, its <code>3<sup>rd</sup></code> lap in <code>3 * 2<sup>2</sup> = 12</code> seconds, etc.

You are also given an integer `changeTime` and an integer `numLaps`.

The race consists of `numLaps` laps and you may start the race with **any** tire. You have an **unlimited** supply of each tire and after every lap, you may **change** to any given tire (including the current tire type) if you wait `changeTime` seconds.

Return *the **minimum** time to finish the race*.

#### Example 1:
<pre>
<strong>Input:</strong> tires = [[2,3],[3,4]], changeTime = 5, numLaps = 4
<strong>Output:</strong> 21
<strong>Explanation:</strong>
Lap 1: Start with tire 0 and finish the lap in 2 seconds.
Lap 2: Continue with tire 0 and finish the lap in 2 * 3 = 6 seconds.
Lap 3: Change tires to a new tire 0 for 5 seconds and then finish the lap in another 2 seconds.
Lap 4: Continue with tire 0 and finish the lap in 2 * 3 = 6 seconds.
Total time = 2 + 6 + 5 + 2 + 6 = 21 seconds.
The minimum time to complete the race is 21 seconds.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> tires = [[1,10],[2,2],[3,4]], changeTime = 6, numLaps = 5
<strong>Output:</strong> 25
<strong>Explanation:</strong>
Lap 1: Start with tire 1 and finish the lap in 2 seconds.
Lap 2: Continue with tire 1 and finish the lap in 2 * 2 = 4 seconds.
Lap 3: Change tires to a new tire 1 for 6 seconds and then finish the lap in another 2 seconds.
Lap 4: Continue with tire 1 and finish the lap in 2 * 2 = 4 seconds.
Lap 5: Change tires to tire 0 for 6 seconds then finish the lap in another 1 second.
Total time = 2 + 4 + 6 + 2 + 4 + 6 + 1 = 25 seconds.
The minimum time to complete the race is 25 seconds.
</pre>

#### Constraints:
* <code>1 <= tires.length <= 10<sup>5</sup></code>
* `tires[i].length == 2`
* <code>1 <= f<sub>i</sub>, changeTime <= 10<sup>5</sup></code>
* <code>2 <= r<sub>i</sub> <= 105</sup></code>
* `1 <= numLaps <= 1000`

## Solutions (Rust)

### 1. Solution
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
