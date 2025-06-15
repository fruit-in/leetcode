# 2365. 任务调度器 II
给你一个下标从 **0** 开始的正整数数组 `tasks` ，表示需要 **按顺序** 完成的任务，其中 `tasks[i]` 表示第 `i` 件任务的 **类型** 。

同时给你一个正整数 `space` ，表示一个任务完成 **后** ，另一个 **相同** 类型任务完成前需要间隔的 **最少** 天数。

在所有任务完成前的每一天，你都必须进行以下两种操作中的一种：
* 完成 `tasks` 中的下一个任务
* 休息一天

请你返回完成所有任务所需的 **最少** 天数。

#### 示例 1:
<pre>
<strong>输入:</strong> tasks = [1,2,1,2,3,1], space = 3
<strong>输出:</strong> 9
<strong>解释:</strong>
9 天完成所有任务的一种方法是：
第 1 天：完成任务 0 。
第 2 天：完成任务 1 。
第 3 天：休息。
第 4 天：休息。
第 5 天：完成任务 2 。
第 6 天：完成任务 3 。
第 7 天：休息。
第 8 天：完成任务 4 。
第 9 天：完成任务 5 。
可以证明无法少于 9 天完成所有任务。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> tasks = [5,8,8,5], space = 2
<strong>输出:</strong> 6
<strong>解释:</strong>
6 天完成所有任务的一种方法是：
第 1 天：完成任务 0 。
第 2 天：完成任务 1 。
第 3 天：休息。
第 4 天：休息。
第 5 天：完成任务 2 。
第 6 天：完成任务 3 。
可以证明无法少于 6 天完成所有任务。
</pre>

#### 提示:
* <code>1 <= tasks.length <= 10<sup>5</sup></code>
* <code>1 <= tasks[i] <= 10<sup>9</sup></code>
* `1 <= space <= tasks.length`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn task_scheduler_ii(tasks: Vec<i32>, space: i32) -> i64 {
        let space = space as i64;
        let mut available_day = HashMap::new();
        let mut ret = 0;

        for i in 0..tasks.len() {
            ret = (ret + 1).max(*available_day.get(&tasks[i]).unwrap_or(&0));
            available_day.insert(tasks[i], ret + space + 1);
        }

        ret
    }
}
```
