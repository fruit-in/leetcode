# 1665. 完成所有任务的最少初始能量
给你一个任务数组 `tasks` ，其中 <code>tasks[i] = [actual<sub>i</sub>, minimum<sub>i</sub>]</code> ：
* <code>actual<sub>i</sub></code> 是完成第 `i` 个任务 **需要耗费** 的实际能量。
* <code>minimum<sub>i</sub></code> 是开始第 `i` 个任务前需要达到的最低能量。

比方说，如果任务为 `[10, 12]` 且你当前的能量为 `11` ，那么你不能开始这个任务。如果你当前的能量为 `13` ，你可以完成这个任务，且完成它后剩余能量为 `3` 。

你可以按照 **任意顺序** 完成任务。

请你返回完成所有任务的 **最少** 初始能量。

#### 示例 1:
<pre>
<strong>输入:</strong> tasks = [[1,2],[2,4],[4,8]]
<strong>输出:</strong> 8
<strong>解释:</strong>
一开始有 8 能量，我们按照如下顺序完成任务：
    - 完成第 3 个任务，剩余能量为 8 - 4 = 4 。
    - 完成第 2 个任务，剩余能量为 4 - 2 = 2 。
    - 完成第 1 个任务，剩余能量为 2 - 1 = 1 。
注意到尽管我们有能量剩余，但是如果一开始只有 7 能量是不能完成所有任务的，因为我们无法开始第 3 个任务。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> tasks = [[1,3],[2,4],[10,11],[10,12],[8,9]]
<strong>输出:</strong> 32
<strong>解释:</strong>
一开始有 32 能量，我们按照如下顺序完成任务：
    - 完成第 1 个任务，剩余能量为 32 - 1 = 31 。
    - 完成第 2 个任务，剩余能量为 31 - 2 = 29 。
    - 完成第 3 个任务，剩余能量为 29 - 10 = 19 。
    - 完成第 4 个任务，剩余能量为 19 - 10 = 9 。
    - 完成第 5 个任务，剩余能量为 9 - 8 = 1 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> tasks = [[1,7],[2,8],[3,9],[4,10],[5,11],[6,12]]
<strong>输出:</strong> 27
<strong>解释:</strong>
一开始有 27 能量，我们按照如下顺序完成任务：
    - 完成第 5 个任务，剩余能量为 27 - 5 = 22 。
    - 完成第 2 个任务，剩余能量为 22 - 2 = 20 。
    - 完成第 3 个任务，剩余能量为 20 - 3 = 17 。
    - 完成第 1 个任务，剩余能量为 17 - 1 = 16 。
    - 完成第 4 个任务，剩余能量为 16 - 4 = 12 。
    - 完成第 6 个任务，剩余能量为 12 - 6 = 6 。
</pre>

#### 提示:
* <code>1 <= tasks.length <= 10<sup>5</sup></code>
* <code>1 <= actual<sub>i</sub> <= minimum<sub>i</sub> <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_effort(mut tasks: Vec<Vec<i32>>) -> i32 {
        let mut energy = 0;
        let mut ret = 0;

        tasks.sort_unstable_by_key(|t| t[0] - t[1]);

        for task in &tasks {
            if energy < task[1] {
                ret += task[1] - energy;
                energy = task[1];
            }
            energy -= task[0];
        }

        ret
    }
}
```
