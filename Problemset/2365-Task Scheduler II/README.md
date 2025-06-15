# 2365. Task Scheduler II
You are given a **0-indexed** array of positive integers tasks, representing `tasks` that need to be completed **in order**, where `tasks[i]` represents the **type** of the <code>i<sup>th</sup></code> task.

You are also given a positive integer `space`, which represents the **minimum** number of days that must pass **after** the completion of a task before another task of the **same** type can be performed.

Each day, until all tasks have been completed, you must either:
* Complete the next task from `tasks`, or
* Take a break.

Return *the **minimum** number of days needed to complete all tasks*.

#### Example 1:
<pre>
<strong>Input:</strong> tasks = [1,2,1,2,3,1], space = 3
<strong>Output:</strong> 9
<strong>Explanation:</strong>
One way to complete all tasks in 9 days is as follows:
Day 1: Complete the 0th task.
Day 2: Complete the 1st task.
Day 3: Take a break.
Day 4: Take a break.
Day 5: Complete the 2nd task.
Day 6: Complete the 3rd task.
Day 7: Take a break.
Day 8: Complete the 4th task.
Day 9: Complete the 5th task.
It can be shown that the tasks cannot be completed in less than 9 days.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> tasks = [5,8,8,5], space = 2
<strong>Output:</strong> 6
<strong>Explanation:</strong>
One way to complete all tasks in 6 days is as follows:
Day 1: Complete the 0th task.
Day 2: Complete the 1st task.
Day 3: Take a break.
Day 4: Take a break.
Day 5: Complete the 2nd task.
Day 6: Complete the 3rd task.
It can be shown that the tasks cannot be completed in less than 6 days.
</pre>

#### Constraints:
* <code>1 <= tasks.length <= 10<sup>5</sup></code>
* <code>1 <= tasks[i] <= 10<sup>9</sup></code>
* `1 <= space <= tasks.length`

## Solutions (Rust)

### 1. Solution
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
