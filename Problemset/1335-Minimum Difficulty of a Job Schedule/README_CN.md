# 1335. 工作计划的最低难度
你需要制定一份 `d` 天的工作计划表。工作之间存在依赖，要想执行第 `i` 项工作，你必须完成全部 `j` 项工作（ `0 <= j < i`）。

你每天 **至少** 需要完成一项任务。工作计划的总难度是这 `d` 天每一天的难度之和，而一天的工作难度是当天应该完成工作的最大难度。

给你一个整数数组 `jobDifficulty` 和一个整数 `d`，分别代表工作难度和需要计划的天数。第 `i` 项工作的难度是 `jobDifficulty[i]`。

返回整个工作计划的 **最小难度** 。如果无法制定工作计划，则返回 **-1** 。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/01/16/untitled.png)
<pre>
<strong>输入:</strong> jobDifficulty = [6,5,4,3,2,1], d = 2
<strong>输出:</strong> 7
<strong>解释:</strong> 第一天，您可以完成前 5 项工作，总难度 = 6.
第二天，您可以完成最后一项工作，总难度 = 1.
计划表的难度 = 6 + 1 = 7
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> jobDifficulty = [9,9,9], d = 4
<strong>输出:</strong> -1
<strong>解释:</strong> 就算你每天完成一项工作，仍然有一天是空闲的，你无法制定一份能够满足既定工作时间的计划表。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> jobDifficulty = [1,1,1], d = 3
<strong>输出:</strong> 3
<strong>解释:</strong> 工作计划为每天一项工作，总难度为 3 。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> jobDifficulty = [7,1,7,1,7,1], d = 3
<strong>输出:</strong> 15
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> jobDifficulty = [11,111,22,222,33,333,44,444], d = 6
<strong>输出:</strong> 843
</pre>

#### 提示:
* `1 <= jobDifficulty.length <= 300`
* `0 <= jobDifficulty[i] <= 1000`
* `1 <= d <= 10`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let n = job_difficulty.len();
        let d = d as usize;

        if n < d {
            return -1;
        }

        let mut dp = vec![vec![i32::MAX; n + 1]; d + 1];
        let mut max_difficulty = 0;

        for i in 0..=n - d {
            max_difficulty = max_difficulty.max(job_difficulty[i]);
            dp[1][i + 1] = max_difficulty;
        }

        for i in 1..d {
            for j in i..=n + i - d {
                max_difficulty = 0;

                for k in 1..=n + i + 1 - j - d {
                    max_difficulty = max_difficulty.max(job_difficulty[j + k - 1]);
                    dp[i + 1][j + k] = dp[i + 1][j + k].min(dp[i][j] + max_difficulty);
                }
            }
        }

        dp[d][n]
    }
}
```
