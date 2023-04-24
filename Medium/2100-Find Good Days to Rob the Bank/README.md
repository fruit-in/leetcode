# 2100. Find Good Days to Rob the Bank
You and a gang of thieves are planning on robbing a bank. You are given a **0-indexed** integer array `security`, where `security[i]` is the number of guards on duty on the <code>i<sup>th</sup></code> day. The days are numbered starting from `0`. You are also given an integer `time`.

The <code>i<sup>th</sup></code> day is a good day to rob the bank if:

* There are at least `time` days before and after the <code>i<sup>th</sup></code> day,
* The number of guards at the bank for the `time` days **before** `i` are **non-increasing**, and
* The number of guards at the bank for the `time` days **after** `i` are **non-decreasing**.

More formally, this means day `i` is a good day to rob the bank if and only if `security[i - time] >= security[i - time + 1] >= ... >= security[i] <= ... <= security[i + time - 1] <= security[i + time]`.

Return *a list of **all** days (**0-indexed**) that are good days to rob the bank*. *The order that the days are returned in does **not** matter*.

#### Example 1:
<pre>
<strong>Input:</strong> security = [5,3,3,3,5,6,2], time = 2
<strong>Output:</strong> [2,3]
<strong>Explanation:</strong>
On day 2, we have security[0] >= security[1] >= security[2] <= security[3] <= security[4].
On day 3, we have security[1] >= security[2] >= security[3] <= security[4] <= security[5].
No other days satisfy this condition, so days 2 and 3 are the only good days to rob the bank.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> security = [1,1,1,1,1], time = 0
<strong>Output:</strong> [0,1,2,3,4]
<strong>Explanation:</strong>
Since time equals 0, every day is a good day to rob the bank, so return every day.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> security = [1,2,3,4,5,6], time = 2
<strong>Output:</strong> []
<strong>Explanation:</strong>
No day has 2 days before it that have a non-increasing number of guards.
Thus, no day is a good day to rob the bank, so return an empty list.
</pre>

#### Constraints:
* <code>1 <= security.length <= 10<sup>5</sup></code>
* <code>0 <= security[i], time <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
        let mut prefix_l = vec![0; security.len()];
        let mut prefix_r = vec![0; security.len()];

        for i in 1..security.len() {
            if security[i] <= security[i - 1] {
                prefix_l[i] = prefix_l[i - 1] + 1;
            }
            if security[security.len() - i - 1] <= security[security.len() - i] {
                prefix_r[security.len() - i - 1] = prefix_r[security.len() - i] + 1;
            }
        }

        (time..security.len() as i32 - time)
            .filter(|&i| prefix_l[i as usize] >= time && prefix_r[i as usize] >= time)
            .collect()
    }
}
```
