# 621. 任务调度器
给你一个用字符数组 `tasks` 表示的 CPU 需要执行的任务列表。其中每个字母表示一种不同种类的任务。任务可以以任意顺序执行，并且每个任务都可以在 1 个单位时间内执行完。在任何一个单位时间，CPU 可以完成一个任务，或者处于待命状态。

然而，两个 **相同种类** 的任务之间必须有长度为整数 `n` 的冷却时间，因此至少有连续 `n` 个单位时间内 CPU 在执行不同的任务，或者在待命状态。

你需要计算完成所有任务所需要的 **最短时间** 。

#### 示例 1:
<pre>
<strong>输入:</strong> tasks = ["A","A","A","B","B","B"], n = 2
<strong>输出:</strong> 8
<strong>解释:</strong> A -> B -> (待命) -> A -> B -> (待命) -> A -> B
     在本示例中，两个相同类型任务之间必须间隔长度为 n = 2 的冷却时间，而执行一个任务只需要一个单位时间，所以中间出现了（待命）状态。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> tasks = ["A","A","A","B","B","B"], n = 0
<strong>输出:</strong> 6
<strong>解释:</strong> 在这种情况下，任何大小为 6 的排列都可以满足要求，因为 n = 0
["A","A","A","B","B","B"]
["A","B","A","B","A","B"]
["B","B","B","A","A","A"]
...
诸如此类
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> tasks = ["A","A","A","A","A","A","B","C","D","E","F","G"], n = 2
<strong>输出:</strong> 16
<strong>解释:</strong> 一种可能的解决方案是：
     A -> B -> C -> A -> D -> E -> A -> F -> G -> A -> (待命) -> (待命) -> A -> (待命) -> (待命) -> A
</pre>

#### 提示:
* <code>1 <= task.length <= 10<sup>4</sup></code>
* `tasks[i]` 是大写英文字母
* `n` 的取值范围为 `[0, 100]`

## 题解 (Rust)

### 1. 题解
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
