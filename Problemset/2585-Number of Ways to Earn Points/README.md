# 2585. Number of Ways to Earn Points
There is a test that has `n` types of questions. You are given an integer `target` and a **0-indexed** 2D integer array `types` where <code>types[i] = [count<sub>i</sub>, marks<sub>i</sub>]</code> indicates that there are <code>count<sub>i</sub></code> questions of the <code>i<sup>th</sup></code> type, and each one of them is worth <code>marks<sub>i</sub></code> points.

Return *the number of ways you can earn **exactly*** `target` *points in the exam*. Since the answer may be too large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

**Note** that questions of the same type are indistinguishable.

* For example, if there are `3` questions of the same type, then solving the <code>1<sup>st</sup></code> and <code>2<sup>nd</sup></code> questions is the same as solving the <code>1<sup>st</sup></code> and <code>3<sup>rd</sup></code> questions, or the <code>2<sup>nd</sup></code> and <code>3<sup>rd</sup></code> questions.

#### Example 1:
<pre>
<strong>Input:</strong> target = 6, types = [[6,1],[3,2],[2,3]]
<strong>Output:</strong> 7
<strong>Explanation:</strong> You can earn 6 points in one of the seven ways:
- Solve 6 questions of the 0th type: 1 + 1 + 1 + 1 + 1 + 1 = 6
- Solve 4 questions of the 0th type and 1 question of the 1st type: 1 + 1 + 1 + 1 + 2 = 6
- Solve 2 questions of the 0th type and 2 questions of the 1st type: 1 + 1 + 2 + 2 = 6
- Solve 3 questions of the 0th type and 1 question of the 2nd type: 1 + 1 + 1 + 3 = 6
- Solve 1 question of the 0th type, 1 question of the 1st type and 1 question of the 2nd type: 1 + 2 + 3 = 6
- Solve 3 questions of the 1st type: 2 + 2 + 2 = 6
- Solve 2 questions of the 2nd type: 3 + 3 = 6
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> target = 5, types = [[50,1],[50,2],[50,5]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> You can earn 5 points in one of the four ways:
- Solve 5 questions of the 0th type: 1 + 1 + 1 + 1 + 1 = 5
- Solve 3 questions of the 0th type and 1 question of the 1st type: 1 + 1 + 1 + 2 = 5
- Solve 1 questions of the 0th type and 2 questions of the 1st type: 1 + 2 + 2 = 5
- Solve 1 question of the 2nd type: 5
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> target = 18, types = [[6,1],[3,2],[2,3]]
<strong>Output:</strong> 1
<strong>Explanation:</strong> You can only earn 18 points by answering all questions.
</pre>

#### Constraints:
* `1 <= target <= 1000`
* `n == types.length`
* `1 <= n <= 50`
* `types[i].length == 2`
* <code>1 <= count<sub>i</sub>, marks<sub>i</sub> <= 50</code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn ways_to_reach_target(target: i32, types: Vec<Vec<i32>>) -> i32 {
        let target = target as usize;
        let mut dp = vec![0; target + 1];
        dp[0] = 1;

        for i in 0..types.len() {
            for j in (0..target).rev() {
                for k in 1..=types[i][0] {
                    let points = (k * types[i][1]) as usize + j;

                    if points > target {
                        break;
                    }

                    dp[points] = (dp[points] + dp[j]) % 1_000_000_007;
                }
            }
        }

        *dp.last().unwrap()
    }
}
```
