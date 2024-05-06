# 1449. Form Largest Integer With Digits That Add up to Target
Given an array of integers `cost` and an integer `target`, return *the **maximum** integer you can paint under the following rules*:

* The cost of painting a digit `(i + 1)` is given by `cost[i]` (**0-indexed**).
* The total cost used must be equal to `target`.
* The integer does not have `0` digits.

Since the answer may be very large, return it as a string. If there is no way to paint any integer given the condition, return `"0"`.

#### Example 1:
<pre>
<strong>Input:</strong> cost = [4,3,2,5,6,7,2,5,5], target = 9
<strong>Output:</strong> "7772"
<strong>Explanation:</strong> The cost to paint the digit '7' is 2, and the digit '2' is 3. Then cost("7772") = 2*3+ 3*1 = 9. You could also paint "977", but "7772" is the largest number.
Digit    cost
  1  ->   4
  2  ->   3
  3  ->   2
  4  ->   5
  5  ->   6
  6  ->   7
  7  ->   2
  8  ->   5
  9  ->   5
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> cost = [7,6,5,5,5,6,8,7,8], target = 12
<strong>Output:</strong> "85"
<strong>Explanation:</strong> The cost to paint the digit '8' is 7, and the digit '5' is 5. Then cost("85") = 7 + 5 = 12.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> cost = [2,4,6,2,4,6,4,4,4], target = 5
<strong>Output:</strong> "0"
<strong>Explanation:</strong> It is impossible to paint any integer with total cost equal to target.
</pre>

#### Constraints:
* `cost.length == 9`
* `1 <= cost[i], target <= 5000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn largest_number(cost: Vec<i32>, target: i32) -> String {
        let target = target as usize;
        let cost = cost.iter().map(|&x| x as usize).collect::<Vec<_>>();
        let mut dp = vec![[-1; 10]; target + 1];
        dp[0] = [0; 10];

        for i in 0..=target {
            if dp[i][0] == -1 {
                continue;
            }

            for j in 0..9 {
                let mut count = dp[i];
                count[9 - j] += 1;
                count[0] += 1;

                if i + cost[j] <= target && dp[i + cost[j]] < count {
                    dp[i + cost[j]] = count;
                }
            }
        }

        if dp[target][0] == -1 {
            return "0".to_string();
        }

        (0..9)
            .rev()
            .map(|i| vec![std::char::from_u32(49 + i as u32).unwrap(); dp[target][9 - i] as usize])
            .flatten()
            .collect()
    }
}
```
