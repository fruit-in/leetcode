# 743. 网络延迟时间
有 `N` 个网络节点，标记为 `1` 到 `N`。

给定一个列表 `times`，表示信号经过**有向**边的传递时间。 `times[i] = (u, v, w)`，其中 `u` 是源节点，`v` 是目标节点， `w` 是一个信号从源节点传递到目标节点的时间。

现在，我们从某个节点 `K` 发出一个信号。需要多久才能使所有节点都收到信号？如果不能使所有节点收到信号，返回 `-1`。

#### 示例:
![](https://assets.leetcode.com/uploads/2019/05/23/931_example_1.png)
<pre>
<strong>输入:</strong> times = [[2,1,1],[2,3,1],[3,4,1]], N = 4, K = 2
<strong>输出:</strong> 2
</pre>

#### 注意:
1. `N` 的范围在 `[1, 100]` 之间。
2. `K` 的范围在 `[1, N]` 之间。
3. `times` 的长度在 `[1, 6000]` 之间。
4. 所有的边 `times[i] = (u, v, w)` 都有 `1 <= u, v <= N` 且 `0 <= w <= 100`。

## 题解 (Rust)

### 1. 深度优先搜索
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
