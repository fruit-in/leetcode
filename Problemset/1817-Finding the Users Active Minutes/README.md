# 1817. Finding the Users Active Minutes
You are given the logs for users' actions on LeetCode, and an integer `k`. The logs are represented by a 2D integer array `logs` where each `logs[i] = [IDi, timei]` indicates that the user with `IDi` performed an action at the minute `timei`.

**Multiple users** can perform actions simultaneously, and a single user can perform **multiple actions** in the same minute.

The **user active minutes (UAM)** for a given user is defined as the **number of unique minutes** in which the user performed an action on LeetCode. A minute can only be counted once, even if multiple actions occur during it.

You are to calculate a **1-indexed** array `answer` of size `k` such that, for each j (`1 <= j <= k`), `answer[j]` is the **number of users** whose **UAM** equals `j`.

Return *the array* `answer` *as described above*.

#### Example 1:
<pre>
<strong>Input:</strong> logs = [[0,5],[1,2],[0,2],[0,5],[1,3]], k = 5
<strong>Output:</strong> [0,2,0,0,0]
<strong>Explanation:</strong>
The user with ID=0 performed actions at minutes 5, 2, and 5 again. Hence, they have a UAM of 2 (minute 5 is only counted once).
The user with ID=1 performed actions at minutes 2 and 3. Hence, they have a UAM of 2.
Since both users have a UAM of 2, answer[2] is 2, and the remaining answer[j] values are 0.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> logs = [[1,1],[2,2],[2,3]], k = 4
<strong>Output:</strong> [1,1,0,0]
<strong>Explanation:</strong>
The user with ID=1 performed a single action at minute 1. Hence, they have a UAM of 1.
The user with ID=2 performed actions at minutes 2 and 3. Hence, they have a UAM of 2.
There is one user with a UAM of 1 and one with a UAM of 2.
Hence, answer[1] = 1, answer[2] = 1, and the remaining values are 0.
</pre>

#### Constraints:
* <code>1 <= logs.length <= 10<sup>4</sup></code>
* <code>0 <= IDi <= 10<sup>9</sup></code>
* <code>1 <= timei <= 10<sup>5</sup></code>
* `k` is in the range <code>[The maximum UAM for a user, 10<sup>5</sup>]</code>.

## Solutions (Rust)

### 1. Solution
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
