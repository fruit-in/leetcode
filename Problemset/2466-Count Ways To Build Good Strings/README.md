# 2466. Count Ways To Build Good Strings
Given the integers `zero`, `one`, `low`, and `high`, we can construct a string by starting with an empty string, and then at each step perform either of the following:

* Append the character `'0'` `zero` times.
* Append the character `'1'` `one` times.

This can be performed any number of times.

A **good** string is a string constructed by the above process having a **length** between `low` and `high` (**inclusive**).

Return *the number of **different** good strings that can be constructed satisfying these properties*. Since the answer can be large, return it **modulo** <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> low = 3, high = 3, zero = 1, one = 1
<strong>Output:</strong> 8
<strong>Explanation:</strong>
One possible valid good string is "011".
It can be constructed as follows: "" -> "0" -> "01" -> "011".
All binary strings from "000" to "111" are good strings in this example.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> low = 2, high = 3, zero = 1, one = 2
<strong>Output:</strong> 5
<strong>Explanation:</strong> The good strings are "00", "11", "000", "110", and "011".
</pre>

#### Constraints:
* <code>1 <= low <= high <= 10<sup>5</sup></code>
* `1 <= zero, one <= low`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let (low, high) = (low as usize, high as usize);
        let (zero, one) = (zero as usize, one as usize);
        let mut dp = vec![0; high + 1];
        let mut ret = 0;
        dp[0] = 1;

        for i in 0..=high {
            if i >= low {
                ret = (ret + dp[i]) % 1_000_000_007;
            }

            if i + zero <= high {
                dp[i + zero] = (dp[i + zero] + dp[i]) % 1_000_000_007;
            }
            if i + one <= high {
                dp[i + one] = (dp[i + one] + dp[i]) % 1_000_000_007;
            }
        }

        ret
    }
}
```
