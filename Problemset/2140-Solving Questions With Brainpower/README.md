# 2140. Solving Questions With Brainpower
You are given a **0-indexed** 2D integer array `questions` where <code>questions[i] = [points<sub>i</sub>, brainpower<sub>i</sub>]</code>.

The array describes the questions of an exam, where you have to process the questions **in order** (i.e., starting from question `0`) and make a decision whether to **solve** or **skip** each question. Solving question `i` will **earn** you <code>points<sub>i</sub></code> points but you will be **unable** to solve each of the next <code>brainpower<sub>i</sub></code> questions. If you skip question `i`, you get to make the decision on the next question.

* For example, given `questions = [[3, 2], [4, 3], [4, 4], [2, 5]]`:
    * If question `0` is solved, you will earn `3` points but you will be unable to solve questions `1` and `2`.
    * If instead, question `0` is skipped and question `1` is solved, you will earn `4` points but you will be unable to solve questions `2` and `3`.

Return *the **maximum** points you can earn for the exam*.

#### Example 1:
<pre>
<strong>Input:</strong> questions = [[3,2],[4,3],[4,4],[2,5]]
<strong>Output:</strong> 5
<strong>Explanation:</strong> The maximum points can be earned by solving questions 0 and 3.
- Solve question 0: Earn 3 points, will be unable to solve the next 2 questions
- Unable to solve questions 1 and 2
- Solve question 3: Earn 2 points
Total points earned: 3 + 2 = 5. There is no other way to earn 5 or more points.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> questions = [[1,1],[2,2],[3,3],[4,4],[5,5]]
<strong>Output:</strong> 7
<strong>Explanation:</strong> The maximum points can be earned by solving questions 1 and 4.
- Skip question 0
- Solve question 1: Earn 2 points, will be unable to solve the next 2 questions
- Unable to solve questions 2 and 3
- Solve question 4: Earn 5 points
Total points earned: 2 + 5 = 7. There is no other way to earn 7 or more points.
</pre>

#### Constraints:
* <code>1 <= questions.length <= 10<sup>5</sup></code>
* `questions[i].length == 2`
* <code>1 <= points<sub>i</sub>, brainpower<sub>i</sub> <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
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
