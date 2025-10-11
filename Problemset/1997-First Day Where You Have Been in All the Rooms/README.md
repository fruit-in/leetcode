# 1997. First Day Where You Have Been in All the Rooms
There are `n` rooms you need to visit, labeled from `0` to `n - 1`. Each day is labeled, starting from `0`. You will go in and visit one room a day.

Initially on day `0`, you visit room `0`. The **order** you visit the rooms for the coming days is determined by the following **rules** and a given **0-indexed** array `nextVisit` of length `n`:
* Assuming that on a day, you visit room `i`,
* if you have been in room `i` an **odd** number of times (**including** the current visit), on the **next** day you will visit a room with a **lower or equal room number** specified by `nextVisit[i]` where `0 <= nextVisit[i] <= i`;
* if you have been in room `i` an **even** number of times (**including** the current visit), on the **next** day you will visit room `(i + 1) mod n`.

Return *the label of the first day where you have been in all the rooms*. It can be shown that such a day exists. Since the answer may be very large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> nextVisit = [0,0]
<strong>Output:</strong> 2
<strong>Explanation:</strong>
- On day 0, you visit room 0. The total times you have been in room 0 is 1, which is odd.
  On the next day you will visit room nextVisit[0] = 0
- On day 1, you visit room 0, The total times you have been in room 0 is 2, which is even.
  On the next day you will visit room (0 + 1) mod 2 = 1
- On day 2, you visit room 1. This is the first day where you have been in all the rooms.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nextVisit = [0,0,2]
<strong>Output:</strong> 6
<strong>Explanation:</strong>
Your room visiting order for each day is: [0,0,1,0,0,1,2,...].
Day 6 is the first day where you have been in all the rooms.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nextVisit = [0,1,2,0]
<strong>Output:</strong> 6
<strong>Explanation:</strong>
Your room visiting order for each day is: [0,0,1,1,2,2,3,...].
Day 6 is the first day where you have been in all the rooms.
</pre>

#### Constraints:
* `n == nextVisit.length`
* <code>2 <= n <= 10<sup>5</sup></code>
* `0 <= nextVisit[i] <= i`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
        let next_visit = next_visit.iter().map(|&x| x as usize).collect::<Vec<_>>();
        let n = next_visit.len();
        let mut dp = vec![[0, 1_i32]; n];

        for i in 1..n {
            dp[i][0] = (dp[i - 1][0] + dp[i - 1][1] + 1) % 1_000_000_007;
            if next_visit[i] != i {
                dp[i][1] = dp[i - 1][0] - dp[next_visit[i]][0] + dp[i - 1][1] + 2;
                dp[i][1] = dp[i][1].rem_euclid(1_000_000_007);
            }
        }

        dp[n - 1][0]
    }
}
```
