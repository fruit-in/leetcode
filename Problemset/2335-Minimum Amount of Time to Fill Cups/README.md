# 2335. Minimum Amount of Time to Fill Cups
You have a water dispenser that can dispense cold, warm, and hot water. Every second, you can either fill up `2` cups with **different** types of water, or `1` cup of any type of water.

You are given a **0-indexed** integer array `amount` of length `3` where `amount[0]`, `amount[1]`, and `amount[2]` denote the number of cold, warm, and hot water cups you need to fill respectively. Return *the **minimum** number of seconds needed to fill up all the cups*.

#### Example 1:
<pre>
<strong>Input:</strong> amount = [1,4,2]
<strong>Output:</strong> 4
<strong>Explanation:</strong> One way to fill up the cups is:
Second 1: Fill up a cold cup and a warm cup.
Second 2: Fill up a warm cup and a hot cup.
Second 3: Fill up a warm cup and a hot cup.
Second 4: Fill up a warm cup.
It can be proven that 4 is the minimum number of seconds needed.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> amount = [5,4,4]
<strong>Output:</strong> 7
<strong>Explanation:</strong> One way to fill up the cups is:
Second 1: Fill up a cold cup, and a hot cup.
Second 2: Fill up a cold cup, and a warm cup.
Second 3: Fill up a cold cup, and a warm cup.
Second 4: Fill up a warm cup, and a hot cup.
Second 5: Fill up a cold cup, and a hot cup.
Second 6: Fill up a cold cup, and a warm cup.
Second 7: Fill up a hot cup.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> amount = [5,0,0]
<strong>Output:</strong> 5
<strong>Explanation:</strong> Every second, we fill up a cold cup.
</pre>

#### Constraints:
* `amount.length == 3`
* `0 <= amount[i] <= 100`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        let mut amount = amount;
        amount.sort_unstable();

        if amount[0] + amount[1] < amount[2] {
            amount[2]
        } else {
            (amount.iter().sum::<i32>() + 1) / 2
        }
    }
}
```
