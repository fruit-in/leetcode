# 1235. 规划兼职工作
你打算利用空闲时间来做兼职工作赚些零花钱。

这里有 `n` 份兼职工作，每份工作预计从 `startTime[i]` 开始到 `endTime[i]` 结束，报酬为 `profit[i]`。

给你一份兼职工作表，包含开始时间 `startTime`，结束时间 `endTime` 和预计报酬 `profit` 三个数组，请你计算并返回可以获得的最大报酬。

注意，时间上出现重叠的 2 份工作不能同时进行。

如果你选择的工作在时间 `X` 结束，那么你可以立刻进行在时间 `X` 开始的下一份工作。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2019/10/10/sample1_1584.png)
<pre>
<strong>输入:</strong> startTime = [1,2,3,3], endTime = [3,4,5,6], profit = [50,10,40,70]
<strong>输出:</strong> 120
<strong>解释:</strong>
我们选出第 1 份和第 4 份工作，
时间范围是 [1-3]+[3-6]，共获得报酬 120 = 50 + 70。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2019/10/10/sample22_1584.png)
<pre>
<strong>输入:</strong> startTime = [1,2,3,4,6], endTime = [3,5,10,6,9], profit = [20,20,100,70,60]
<strong>输出:</strong> 150
<strong>解释:</strong>
我们选择第 1，4，5 份工作。
共获得报酬 150 = 20 + 70 + 60。
</pre>

#### 示例 3:
![](https://assets.leetcode.com/uploads/2019/10/10/sample3_1584.png)
<pre>
<strong>输入:</strong> startTime = [1,1,1], endTime = [2,3,4], profit = [5,6,4]
<strong>输出:</strong> 6
</pre>

#### 提示:
* <code>1 <= startTime.length == endTime.length == profit.length <= 5 * 10<sup>4</sup></code>
* <code>1 <= startTime[i] < endTime[i] <= 10<sup>9</sup></code>
* <code>1 <= profit[i] <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs = (0..profit.len())
            .map(|i| (end_time[i], start_time[i], profit[i]))
            .collect::<Vec<_>>();
        let mut dp = vec![(0, 0)];
        let mut ret = 0;
        jobs.sort_unstable();

        for &(et, st, pf) in &jobs {
            let i = match dp.binary_search_by_key(&st, |&(t, _)| t) {
                Ok(i) => i,
                Err(i) => i - 1,
            };

            if et != dp.last().unwrap().0 {
                dp.push((et, dp[i].1 + pf));
            } else if dp.last().unwrap().1 < dp[i].1 + pf {
                dp.last_mut().unwrap().1 = dp[i].1 + pf;
            }

            if ret > dp.last().unwrap().1 {
                dp.last_mut().unwrap().1 = ret;
            } else {
                ret = dp.last().unwrap().1;
            }
        }

        ret
    }
}
```
