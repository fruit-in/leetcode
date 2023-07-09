# 2432. 处理用时最长的那个任务的员工
共有 `n` 位员工，每位员工都有一个从 `0` 到 `n - 1` 的唯一 id 。

给你一个二维整数数组 `logs` ，其中 <code>logs[i] = [id<sub>i</sub>, leaveTime<sub>i</sub>]</code> ：

* <code>id<sub>i</sub></code> 是处理第 `i` 个任务的员工的 id ，且
* <code>leaveTime<sub>i</sub></code> 是员工完成第 `i` 个任务的时刻。所有 <code>leaveTime<sub>i</sub></code> 的值都是 **唯一** 的。

注意，第 `i` 个任务在第 `(i - 1)` 个任务结束后立即开始，且第 `0` 个任务从时刻 `0` 开始。

返回处理用时最长的那个任务的员工的 id 。如果存在两个或多个员工同时满足，则返回几人中 **最小** 的 id 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 10, logs = [[0,3],[2,5],[0,9],[1,15]]
<strong>输出:</strong> 1
<strong>解释:</strong>
任务 0 于时刻 0 开始，且在时刻 3 结束，共计 3 个单位时间。
任务 1 于时刻 3 开始，且在时刻 5 结束，共计 2 个单位时间。
任务 2 于时刻 5 开始，且在时刻 9 结束，共计 4 个单位时间。
任务 3 于时刻 9 开始，且在时刻 15 结束，共计 6 个单位时间。
时间最长的任务是任务 3 ，而 id 为 1 的员工是处理此任务的员工，所以返回 1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 26, logs = [[1,1],[3,7],[2,12],[7,17]]
<strong>输出:</strong> 3
<strong>解释:</strong>
任务 0 于时刻 0 开始，且在时刻 1 结束，共计 1 个单位时间。
任务 1 于时刻 1 开始，且在时刻 7 结束，共计 6 个单位时间。
任务 2 于时刻 7 开始，且在时刻 12 结束，共计 5 个单位时间。
任务 3 于时刻 12 开始，且在时刻 17 结束，共计 5 个单位时间。
时间最长的任务是任务 1 ，而 id 为 3 的员工是处理此任务的员工，所以返回 3 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 2, logs = [[0,10],[1,20]]
<strong>输出:</strong> 0
<strong>解释:</strong>
任务 0 于时刻 0 开始，且在时刻 10 结束，共计 10 个单位时间。
任务 1 于时刻 10 开始，且在时刻 20 结束，共计 10 个单位时间。
时间最长的任务是任务 0 和 1 ，处理这两个任务的员工的 id 分别是 0 和 1 ，所以返回最小的 0 。
</pre>

#### 提示:
* `2 <= n <= 500`
* `1 <= logs.length <= 500`
* `logs[i].length == 2`
* <code>0 <= id<sub>i</sub> <= n - 1</code>
* <code>1 <= leaveTime<sub>i</sub> <= 500</code>
* <code>id<sub>i</sub> != id<sub>i+1</sub></code>
* <code>leaveTime<sub>i</sub></code> 按严格递增顺序排列

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn hardest_worker(n: i32, logs: Vec<Vec<i32>>) -> i32 {
        let mut start_time = 0;
        let mut max_time = 0;
        let mut ret = 0;

        for i in 0..logs.len() {
            let time = logs[i][1] - start_time;

            if time > max_time || (time == max_time && logs[i][0] < ret) {
                max_time = time;
                ret = logs[i][0];
            }

            start_time = logs[i][1];
        }

        ret
    }
}
```
