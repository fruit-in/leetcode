# 743. Network Delay Time
There are `N` network nodes, labelled `1` to `N`.

Given `times`, a list of travel times as **directed** edges `times[i] = (u, v, w)`, where `u` is the source node, `v` is the target node, and `w` is the time it takes for a signal to travel from source to target.

Now, we send a signal from a certain node `K`. How long will it take for all nodes to receive the signal? If it is impossible, return `-1`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2019/05/23/931_example_1.png)
<pre>
<strong>Input:</strong> times = [[2,1,1],[2,3,1],[3,4,1]], N = 4, K = 2
<strong>Output:</strong> 2
</pre>

#### Note:
1. `N` will be in the range `[1, 100]`.
2. `K` will be in the range `[1, N]`.
3. The length of `times` will be in the range `[1, 6000]`.
4. All edges `times[i] = (u, v, w)` will have `1 <= u, v <= N` and `0 <= w <= 100`.

## Solutions (Rust)

### 1. DFS
```Rust
impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut to_time = vec![vec![]; n as usize + 1];
        let mut stack = vec![k as usize];
        let mut min_time = vec![std::i32::MAX; n as usize + 1];

        min_time[k as usize] = 0;

        for item in times {
            to_time[item[0] as usize].push((item[1] as usize, item[2]));
        }

        while let Some(from) = stack.pop() {
            for i in 0..to_time[from].len() {
                let (to, time) = to_time[from][i];

                if min_time[to] > min_time[from] + time {
                    min_time[to] = min_time[from] + time;
                    stack.push(to);
                }
            }
        }

        match min_time.into_iter().skip(1).max() {
            Some(std::i32::MAX) => -1,
            Some(t) => t,
            _ => -1,
        }
    }
}
```
