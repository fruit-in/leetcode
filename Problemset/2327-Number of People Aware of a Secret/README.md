# 2327. Number of People Aware of a Secret
On day `1`, one person discovers a secret.

You are given an integer `delay`, which means that each person will **share** the secret with a new person **every day**, starting from `delay` days after discovering the secret. You are also given an integer `forget`, which means that each person will **forget** the secret `forget` days after discovering it. A person **cannot** share the secret on the same day they forgot it, or on any day afterwards.

Given an integer `n`, return *the number of people who know the secret at the end of day* `n`. Since the answer may be very large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> n = 6, delay = 2, forget = 4
<strong>Output:</strong> 5
<strong>Explanation:</strong>
Day 1: Suppose the first person is named A. (1 person)
Day 2: A is the only person who knows the secret. (1 person)
Day 3: A shares the secret with a new person, B. (2 people)
Day 4: A shares the secret with a new person, C. (3 people)
Day 5: A forgets the secret, and B shares the secret with a new person, D. (3 people)
Day 6: B shares the secret with E, and C shares the secret with F. (5 people)
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 4, delay = 1, forget = 3
<strong>Output:</strong> 6
<strong>Explanation:</strong>
Day 1: The first person is named A. (1 person)
Day 2: A shares the secret with B. (2 people)
Day 3: A and B share the secret with 2 new people, C and D. (4 people)
Day 4: A forgets the secret. B, C, and D share the secret with 3 new people. (6 people)
</pre>

#### Constraints:
* `2 <= n <= 1000`
* `1 <= delay < forget <= n`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def peopleAwareOfSecret(self, n: int, delay: int, forget: int) -> int:
        dp = [[0, 0, 0] for _ in range(n + 2)]
        dp[1 + forget][0] = 1
        dp[1 + delay][1] = 1
        dp[1][2] = 1

        for i in range(2, n + 1):
            if i > 1 + forget:
                dp[i][0] = dp[i - forget][1]
            if i > 1 + delay:
                dp[i][1] = dp[i - 1][1] - dp[i][0] + dp[i - delay][1]
            dp[i][2] = dp[i - 1][2] - dp[i][0] + dp[i][1]

        return dp[n][2] % 1000000007
```
