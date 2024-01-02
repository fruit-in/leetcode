# 621. Task Scheduler
Given a characters array `tasks`, representing the tasks a CPU needs to do, where each letter represents a different task. Tasks could be done in any order. Each task is done in one unit of time. For each unit of time, the CPU could complete either one task or just be idle.

However, there is a non-negative integer `n` that represents the cooldown period between two **same tasks** (the same letter in the array), that is that there must be at least `n` units of time between any two same tasks.

Return *the least number of units of times that the CPU will take to finish all the given tasks*.

#### Example 1:
<pre>
<strong>Input:</strong> tasks = ["A","A","A","B","B","B"], n = 2
<strong>Output:</strong> 8
<strong>Explanation:</strong>
A -> B -> idle -> A -> B -> idle -> A -> B
There is at least 2 units of time between any two same tasks.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> tasks = ["A","A","A","B","B","B"], n = 0
<strong>Output:</strong> 6
<strong>Explanation:</strong> On this case any permutation of size 6 would work since n = 0.
["A","A","A","B","B","B"]
["A","B","A","B","A","B"]
["B","B","B","A","A","A"]
...
And so on.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> tasks = ["A","A","A","A","A","A","B","C","D","E","F","G"], n = 2
<strong>Output:</strong> 16
<strong>Explanation:</strong>
One possible solution is
A -> B -> C -> A -> D -> E -> A -> F -> G -> A -> idle -> idle -> A -> idle -> idle -> A
</pre>

#### Constraints:
* <code>1 <= task.length <= 10<sup>4</sup></code>
* `tasks[i]` is upper-case English letter.
* The integer `n` is in the range `[0, 100]`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut count = [0; 27];
        let mut cooldown = [0; 26];
        let mut time = 0;

        for &task in &tasks {
            count[(task as usize) - 65] += 1;
        }

        for _ in 0..tasks.len() {
            let mut min_cooldown = i32::MAX;
            let mut next_task = 26;

            for i in 0..26 {
                if count[i] > 0 {
                    min_cooldown = min_cooldown.min(cooldown[i]);
                }
            }
            time = time.max(min_cooldown);

            for i in 0..26 {
                if cooldown[i] <= time && count[i] > count[next_task] {
                    next_task = i;
                }
            }
            count[next_task] -= 1;
            time += 1;
            cooldown[next_task] = time + n;
        }

        time
    }
}
```
