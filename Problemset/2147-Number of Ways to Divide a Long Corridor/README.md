# 2147. Number of Ways to Divide a Long Corridor
Along a long library corridor, there is a line of seats and decorative plants. You are given a **0-indexed** string `corridor` of length `n` consisting of letters `'S'` and `'P'` where each `'S'` represents a seat and each `'P'` represents a plant.

One room divider has **already** been installed to the left of index `0`, and **another** to the right of index `n - 1`. Additional room dividers can be installed. For each position between indices `i - 1` and `i` (`1 <= i <= n - 1`), at most one divider can be installed.

Divide the corridor into non-overlapping sections, where each section has **exactly two seats** with any number of plants. There may be multiple ways to perform the division. Two ways are **different** if there is a position with a room divider installed in the first way but not in the second way.

Return *the number of ways to divide the corridor*. Since the answer may be very large, return it **modulo** <code>10<sup>9</sup> + 7</code>. If there is no way, return `0`.

#### Example 1:
![](https://assets.leetcode.com/uploads/2021/12/04/1.png)
<pre>
<strong>Input:</strong> corridor = "SSPPSPS"
<strong>Output:</strong> 3
<strong>Explanation:</strong> There are 3 different ways to divide the corridor.
The black bars in the above image indicate the two room dividers already installed.
Note that in each of the ways, each section has exactly two seats.
</pre>

#### Example 2:
![](https://assets.leetcode.com/uploads/2021/12/04/2.png)
<pre>
<strong>Input:</strong> corridor = "PPSPSP"
<strong>Output:</strong> 1
<strong>Explanation:</strong> There is only 1 way to divide the corridor, by not installing any additional dividers.
Installing any would create some section that does not have exactly two seats.
</pre>

#### Example 3:
![](https://assets.leetcode.com/uploads/2021/12/12/3.png)
<pre>
<strong>Input:</strong> corridor = "S"
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no way to divide the corridor because there will always be a section that does not have exactly two seats.
</pre>

#### Constraints:
* `n == corridor.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* `corridor[i]` is either `'S'` or `'P'`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let mut count_s = 0;
        let mut count_p = vec![];

        for c in corridor.chars() {
            if c == 'S' {
                count_s += 1;
                if count_s % 2 == 0 {
                    count_p.push(0_i64);
                }
            } else if count_s > 0 && count_s % 2 == 0 {
                *count_p.last_mut().unwrap() += 1;
            }
        }

        if count_s == 0 || count_s % 2 == 1 {
            return 0;
        }

        count_p.pop();

        count_p
            .iter()
            .fold(1, |acc, x| acc * (x + 1) % 1_000_000_007) as i32
    }
}
```
