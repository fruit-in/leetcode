# 2140. 解决智力问题
给你一个下标从 **0** 开始的二维整数数组 `questions` ，其中 <code>questions[i] = [points<sub>i</sub>, brainpower<sub>i</sub>]</code> 。

这个数组表示一场考试里的一系列题目，你需要 **按顺序** （也就是从问题 `0` 开始依次解决），针对每个问题选择 **解决** 或者 **跳过** 操作。解决问题 `i` 将让你 **获得**  <code>points<sub>i</sub></code> 的分数，但是你将 **无法** 解决接下来的 <code>brainpower<sub>i</sub></code> 个问题（即只能跳过接下来的 <code>brainpower<sub>i</sub></code> 个问题）。如果你跳过问题 `i` ，你可以对下一个问题决定使用哪种操作。

* 比方说，给你 `questions = [[3, 2], [4, 3], [4, 4], [2, 5]]` ：
    * 如果问题 `0` 被解决了， 那么你可以获得 `3` 分，但你不能解决问题 `1` 和 `2` 。
    * 如果你跳过问题 `0` ，且解决问题 `1` ，你将获得 `4` 分但是不能解决问题 `2` 和 `3` 。

请你返回这场考试里你能获得的 **最高** 分数。

#### 示例 1:
<pre>
<strong>输入:</strong> questions = [[3,2],[4,3],[4,4],[2,5]]
<strong>输出:</strong> 5
<strong>解释:</strong> 解决问题 0 和 3 得到最高分。
- 解决问题 0 ：获得 3 分，但接下来 2 个问题都不能解决。
- 不能解决问题 1 和 2
- 解决问题 3 ：获得 2 分
总得分为：3 + 2 = 5 。没有别的办法获得 5 分或者多于 5 分。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> questions = [[1,1],[2,2],[3,3],[4,4],[5,5]]
<strong>输出:</strong> 7
<strong>解释:</strong> 解决问题 1 和 4 得到最高分。
- 跳过问题 0
- 解决问题 1 ：获得 2 分，但接下来 2 个问题都不能解决。
- 不能解决问题 2 和 3
- 解决问题 4 ：获得 5 分
总得分为：2 + 5 = 7 。没有别的办法获得 7 分或者多于 7 分。
</pre>

#### 提示:
* <code>1 <= questions.length <= 10<sup>5</sup></code>
* `questions[i].length == 2`
* <code>1 <= points<sub>i</sub>, brainpower<sub>i</sub> <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut dp = vec![(0, 0); questions.len()];

        for i in (0..dp.len()).rev() {
            let (points, brainpower) = (questions[i][0] as i64, questions[i][1] as usize);

            dp[i].0 = points;
            if i + brainpower + 1 < dp.len() {
                dp[i].0 += dp[i + brainpower + 1].0.max(dp[i + brainpower + 1].1);
            }
            if i + 1 < dp.len() {
                dp[i].1 = dp[i + 1].0.max(dp[i + 1].1);
            }
        }

        dp[0].0.max(dp[0].1)
    }
}
```
