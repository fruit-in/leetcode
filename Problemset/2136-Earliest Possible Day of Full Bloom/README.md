# 2136. Earliest Possible Day of Full Bloom
You have `n` flower seeds. Every seed must be planted first before it can begin to grow, then bloom. Planting a seed takes time and so does the growth of a seed. You are given two **0-indexed** integer arrays `plantTime` and `growTime`, of length `n` each:

* `plantTime[i]` is the number of **full days** it takes you to **plant** the <code>i<sup>th</sup></code> seed. Every day, you can work on planting exactly one seed. You **do not** have to work on planting the same seed on consecutive days, but the planting of a seed is not complete **until** you have worked `plantTime[i]` days on planting it in total.
* `growTime[i]` is the number of **full days** it takes the <code>i<sup>th</sup></code> seed to grow after being completely planted. **After** the last day of its growth, the flower **blooms** and stays bloomed forever.

From the beginning of day `0`, you can plant the seeds in **any** order.

Return *the **earliest** possible day where **all** seeds are blooming*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/12/21/1.png)
<pre>
<strong>Input:</strong> plantTime = [1,4,3], growTime = [2,3,1]
<strong>Output:</strong> 9
<strong>Explanation:</strong> The grayed out pots represent planting days, colored pots represent growing days, and the flower represents the day it blooms.
One optimal way is:
On day 0, plant the 0th seed. The seed grows for 2 full days and blooms on day 3.
On days 1, 2, 3, and 4, plant the 1st seed. The seed grows for 3 full days and blooms on day 8.
On days 5, 6, and 7, plant the 2nd seed. The seed grows for 1 full day and blooms on day 9.
Thus, on day 9, all the seeds are blooming.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/12/21/2.png)
<pre>
<strong>Input:</strong> plantTime = [1,2,3,2], growTime = [2,1,2,1]
<strong>Output:</strong> 9
<strong>Explanation:</strong> The grayed out pots represent planting days, colored pots represent growing days, and the flower represents the day it blooms.
One optimal way is:
On day 1, plant the 0th seed. The seed grows for 2 full days and blooms on day 4.
On days 0 and 3, plant the 1st seed. The seed grows for 1 full day and blooms on day 5.
On days 2, 4, and 5, plant the 2nd seed. The seed grows for 2 full days and blooms on day 8.
On days 6 and 7, plant the 3rd seed. The seed grows for 1 full day and blooms on day 9.
Thus, on day 9, all the seeds are blooming.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> plantTime = [1], growTime = [1]
<strong>Output:</strong> 2
<strong>Explanation:</strong> On day 0, plant the 0th seed. The seed grows for 1 full day and blooms on day 2.
Thus, on day 2, all the seeds are blooming.
</pre>

#### Constraints:
* `n == plantTime.length == growTime.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= plantTime[i], growTime[i] <= 10<sup>4</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut seeds = plant_time.iter().zip(grow_time.iter()).collect::<Vec<_>>();
        let mut day = 0;
        let mut ret = 0;

        seeds.sort_unstable_by_key(|&(_, g)| -g);

        for (p, g) in seeds {
            day += p;
            ret = ret.max(day + g);
        }

        ret
    }
}
```
