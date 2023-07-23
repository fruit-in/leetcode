# 1604. Alert Using Same Key-Card Three or More Times in a One Hour Period
LeetCode company workers use key-cards to unlock office doors. Each time a worker uses their key-card, the security system saves the worker's name and the time when it was used. The system emits an **alert** if any worker uses the key-card **three or more times** in a one-hour period.

You are given a list of strings `keyName` and `keyTime` where `[keyName[i], keyTime[i]]` corresponds to a person's name and the time when their key-card was used **in a single day**.

Access times are given in the **24-hour time format "HH:MM"**, such as `"23:51"` and `"09:49"`.

Return a *list of unique worker names who received an alert for frequent keycard use*. Sort the names in **ascending order alphabetically**.

Notice that `"10:00"` - `"11:00"` is considered to be within a one-hour period, while `"22:51"` - `"23:52"` is not considered to be within a one-hour period.

#### Example 1:
<pre>
<strong>Input:</strong> keyName = ["daniel","daniel","daniel","luis","luis","luis","luis"], keyTime = ["10:00","10:40","11:00","09:00","11:00","13:00","15:00"]
<strong>Output:</strong> ["daniel"]
<strong>Explanation:</strong> "daniel" used the keycard 3 times in a one-hour period ("10:00","10:40", "11:00").
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> keyName = ["alice","alice","alice","bob","bob","bob","bob"], keyTime = ["12:01","12:00","18:00","21:00","21:20","21:30","23:00"]
<strong>Output:</strong> ["bob"]
<strong>Explanation:</strong> "bob" used the keycard 3 times in a one-hour period ("21:00","21:20", "21:30").
</pre>

#### Constraints:
* <code>1 <= keyName.length, keyTime.length <= 10<sup>5</sup></code>
* `keyName.length == keyTime.length`
* `keyTime[i]` is in the format **"HH:MM"**.
* `[keyName[i], keyTime[i]]` is **unique**.
* `1 <= keyName[i].length <= 10`
* `keyName[i] contains only lowercase English letters.`

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;
use std::collections::HashMap;

impl Solution {
    pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
        let mut used_time = HashMap::new();
        let mut ret = BinaryHeap::new();

        for i in 0..key_name.len() {
            let h = key_time[i].get(..2).unwrap().parse::<i32>().unwrap();
            let m = key_time[i].get(3..).unwrap().parse::<i32>().unwrap();

            used_time
                .entry(&key_name[i])
                .or_insert(BinaryHeap::new())
                .push(h * 60 + m);
        }

        for (name, time) in used_time.into_iter() {
            let time = time.into_sorted_vec();

            for i in 2..time.len() {
                if time[i] - time[i - 2] <= 60 {
                    ret.push(name.to_string());
                    break;
                }
            }
        }

        ret.into_sorted_vec()
    }
}
```
