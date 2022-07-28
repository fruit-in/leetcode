# 1817. 查找用户活跃分钟数
给你用户在 LeetCode 的操作日志，和一个整数 `k` 。日志用一个二维整数数组 `logs` 表示，其中每个 `logs[i] = [IDi, timei]` 表示 ID 为 `IDi` 的用户在 `timei` 分钟时执行了某个操作。

**多个用户** 可以同时执行操作，单个用户可以在同一分钟内执行 **多个操作** 。

指定用户的 **用户活跃分钟数（user active minutes，UAM）** 定义为用户对 LeetCode 执行操作的 **唯一分钟数** 。 即使一分钟内执行多个操作，也只能按一分钟计数。

请你统计用户活跃分钟数的分布情况，统计结果是一个长度为 `k` 且 **下标从 1 开始计数** 的数组 `answer` ，对于每个 `j`（`1 <= j <= k`），`answer[j]` 表示 **用户活跃分钟数** 等于 `j` 的用户数。

返回上面描述的答案数组 `answer` 。

#### 示例 1:
<pre>
<strong>输入:</strong> logs = [[0,5],[1,2],[0,2],[0,5],[1,3]], k = 5
<strong>输出:</strong> [0,2,0,0,0]
<strong>解释:</strong>
ID=0 的用户执行操作的分钟分别是：5 、2 和 5 。因此，该用户的用户活跃分钟数为 2（分钟 5 只计数一次）
ID=1 的用户执行操作的分钟分别是：2 和 3 。因此，该用户的用户活跃分钟数为 2
2 个用户的用户活跃分钟数都是 2 ，answer[2] 为 2 ，其余 answer[j] 的值都是 0
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> logs = [[1,1],[2,2],[2,3]], k = 4
<strong>输出:</strong> [1,1,0,0]
<strong>解释:</strong>
ID=1 的用户仅在分钟 1 执行单个操作。因此，该用户的用户活跃分钟数为 1
ID=2 的用户执行操作的分钟分别是：2 和 3 。因此，该用户的用户活跃分钟数为 2
1 个用户的用户活跃分钟数是 1 ，1 个用户的用户活跃分钟数是 2
因此，answer[1] = 1 ，answer[2] = 1 ，其余的值都是 0
</pre>

#### 提示:
* <code>1 <= logs.length <= 10<sup>4</sup></code>
* <code>0 <= IDi <= 10<sup>9</sup></code>
* <code>1 <= timei <= 10<sup>5</sup></code>
* `k` 的取值范围是 <code>[用户的最大用户活跃分钟数, 10<sup>5</sup>]</code>.

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut minutes = HashMap::new();
        let mut answer = vec![0; k as usize];

        for log in logs {
            minutes
                .entry(log[0])
                .or_insert(HashSet::new())
                .insert(log[1]);
        }

        for (id, min) in minutes {
            answer[min.len() - 1] += 1;
        }

        answer
    }
}
```
