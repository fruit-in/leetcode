# 2432. The Employee That Worked on the Longest Task
There are `n` employees, each with a unique id from `0` to `n - 1`.

You are given a 2D integer array `logs` where <code>logs[i] = [id<sub>i</sub>, leaveTime<sub>i</sub>]</code> where:

* <code>id<sub>i</sub></code> is the id of the employee that worked on the <code>i<sup>th</sup></code> task, and
* <code>leaveTime<sub>i</sub></code> is the time at which the employee finished the <code>i<sup>th</sup></code> task. All the values <code>leaveTime<sub>i</sub></code> are **unique**.

Note that the <code>i<sup>th</sup></code> task starts the moment right after the <code>(i - 1)<sup>th</sup></code> task ends, and the <code>0<sup>th</sup></code> task starts at time `0`.

Return *the id of the employee that worked the task with the longest time*. If there is a tie between two or more employees, return *the **smallest** id among them*.

#### Example 1:
<pre>
<strong>Input:</strong> n = 10, logs = [[0,3],[2,5],[0,9],[1,15]]
<strong>Output:</strong> 1
<strong>Explanation:</strong>
Task 0 started at 0 and ended at 3 with 3 units of times.
Task 1 started at 3 and ended at 5 with 2 units of times.
Task 2 started at 5 and ended at 9 with 4 units of times.
Task 3 started at 9 and ended at 15 with 6 units of times.
The task with the longest time is task 3 and the employee with id 1 is the one that worked on it, so we return 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 26, logs = [[1,1],[3,7],[2,12],[7,17]]
<strong>Output:</strong> 3
<strong>Explanation:</strong>
Task 0 started at 0 and ended at 1 with 1 unit of times.
Task 1 started at 1 and ended at 7 with 6 units of times.
Task 2 started at 7 and ended at 12 with 5 units of times.
Task 3 started at 12 and ended at 17 with 5 units of times.
The tasks with the longest time is task 1. The employee that worked on it is 3, so we return 3.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 2, logs = [[0,10],[1,20]]
<strong>Output:</strong> 0
<strong>Explanation:</strong>
Task 0 started at 0 and ended at 10 with 10 units of times.
Task 1 started at 10 and ended at 20 with 10 units of times.
The tasks with the longest time are tasks 0 and 1. The employees that worked on them are 0 and 1, so we return the smallest id 0.
</pre>

#### Constraints:
* `2 <= n <= 500`
* `1 <= logs.length <= 500`
* `logs[i].length == 2`
* <code>0 <= id<sub>i</sub> <= n - 1</code>
* <code>1 <= leaveTime<sub>i</sub> <= 500</code>
* <code>id<sub>i</sub> != id<sub>i+1</sub></code>
* <code>leaveTime<sub>i</sub></code> are sorted in a strictly increasing order.

## Solutions (Rust)

### 1. Solution
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
