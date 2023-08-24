# 1834. 单线程 CPU
给你一个二维数组 `tasks` ，用于表示 `n` 项从 `0` 到 `n - 1` 编号的任务。其中 <code>tasks[i] = [enqueueTime<sub>i</sub>, processingTime<sub>i</sub>]</code> 意味着第 `i` 项任务将会于 <code>enqueueTime<sub>i</sub></code> 时进入任务队列，需要 <code>processingTime<sub>i</sub></code> 的时长完成执行。

现有一个单线程 CPU ，同一时间只能执行 **最多一项** 任务，该 CPU 将会按照下述方式运行：

* 如果 CPU 空闲，且任务队列中没有需要执行的任务，则 CPU 保持空闲状态。
* 如果 CPU 空闲，但任务队列中有需要执行的任务，则 CPU 将会选择 **执行时间最短** 的任务开始执行。如果多个任务具有同样的最短执行时间，则选择下标最小的任务开始执行。
* 一旦某项任务开始执行，CPU 在 **执行完整个任务** 前都不会停止。
* CPU 可以在完成一项任务后，立即开始执行一项新任务。

返回 CPU 处理任务的顺序。

#### 示例 1:
<pre>
<strong>输入:</strong> tasks = [[1,2],[2,4],[3,2],[4,1]]
<strong>输出:</strong> [0,2,3,1]
<strong>解释:</strong> 事件按下述流程运行：
- time = 1 ，任务 0 进入任务队列，可执行任务项 = {0}
- 同样在 time = 1 ，空闲状态的 CPU 开始执行任务 0 ，可执行任务项 = {}
- time = 2 ，任务 1 进入任务队列，可执行任务项 = {1}
- time = 3 ，任务 2 进入任务队列，可执行任务项 = {1, 2}
- 同样在 time = 3 ，CPU 完成任务 0 并开始执行队列中用时最短的任务 2 ，可执行任务项 = {1}
- time = 4 ，任务 3 进入任务队列，可执行任务项 = {1, 3}
- time = 5 ，CPU 完成任务 2 并开始执行队列中用时最短的任务 3 ，可执行任务项 = {1}
- time = 6 ，CPU 完成任务 3 并开始执行任务 1 ，可执行任务项 = {}
- time = 10 ，CPU 完成任务 1 并进入空闲状态
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> tasks = [[7,10],[7,12],[7,5],[7,4],[7,2]]
<strong>输出:</strong> [4,3,2,0,1]
<strong>解释:</strong> 事件按下述流程运行：
- time = 7 ，所有任务同时进入任务队列，可执行任务项  = {0,1,2,3,4}
- 同样在 time = 7 ，空闲状态的 CPU 开始执行任务 4 ，可执行任务项 = {0,1,2,3}
- time = 9 ，CPU 完成任务 4 并开始执行任务 3 ，可执行任务项 = {0,1,2}
- time = 13 ，CPU 完成任务 3 并开始执行任务 2 ，可执行任务项 = {0,1}
- time = 18 ，CPU 完成任务 2 并开始执行任务 0 ，可执行任务项 = {1}
- time = 28 ，CPU 完成任务 0 并开始执行任务 1 ，可执行任务项 = {}
- time = 40 ，CPU 完成任务 1 并进入空闲状态
</pre>

#### 提示:
* `tasks.length == n`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= enqueueTime<sub>i</sub>, processingTime<sub>i</sub> <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut time = 0;
        let mut tasks = tasks.iter().enumerate().collect::<Vec<_>>();
        let mut heap = BinaryHeap::new();
        let mut ret = vec![];

        tasks.sort_unstable_by_key(|(_, t)| -t[0]);

        while !tasks.is_empty() || !heap.is_empty() {
            while let Some(&(i, t)) = tasks.last() {
                if t[0] as i64 <= time {
                    heap.push(Reverse((t[1], i)));
                    tasks.pop();
                } else {
                    break;
                }
            }

            match heap.pop() {
                Some(Reverse((pt, i))) => {
                    time += pt as i64;
                    ret.push(i as i32);
                }
                None => time = tasks.last().unwrap().1[0] as i64,
            }
        }

        ret
    }
}
```
