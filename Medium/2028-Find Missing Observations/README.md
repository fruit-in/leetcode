# 2028. Find Missing Observations
You have observations of `n + m` **6-sided** dice rolls with each face numbered from `1` to `6`. `n` of the observations went missing, and you only have the observations of `m` rolls. Fortunately, you have also calculated the **average value** of the `n + m` rolls.

You are given an integer array `rolls` of length `m` where `rolls[i]` is the value of the <code>i<sup>th</sup></code> observation. You are also given the two integers `mean` and `n`.

Return *an array of length* `n` *containing the missing observations such that the **average value** of the* `n + m` *rolls is **exactly*** `mean`. If there are multiple valid answers, return *any of them*. If no such array exists, return *an empty array*.

The **average value** of a set of `k` numbers is the sum of the numbers divided by `k`.

Note that `mean` is an integer, so the sum of the `n + m` rolls should be divisible by `n + m`.

#### Example 1:
<pre>
<strong>Input:</strong> rolls = [3,2,4,3], mean = 4, n = 2
<strong>Output:</strong> [6,6]
<strong>Explanation:</strong> The mean of all n + m rolls is (3 + 2 + 4 + 3 + 6 + 6) / 6 = 4.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> rolls = [1,5,6], mean = 3, n = 4
<strong>Output:</strong> [2,3,2,2]
<strong>Explanation:</strong> The mean of all n + m rolls is (1 + 5 + 6 + 2 + 3 + 2 + 2) / 7 = 3.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> rolls = [1,2,3,4], mean = 6, n = 4
<strong>Output:</strong> []
<strong>Explanation:</strong> It is impossible for the mean to be 6 no matter what the 4 missing rolls are.
</pre>

#### Constraints:
* `m == rolls.length`
* <code>1 <= n, m <= 10<sup>5</sup></code>
* `1 <= rolls[i], mean <= 6`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let mut n_sum = mean * (rolls.len() as i32 + n) - rolls.into_iter().sum::<i32>();
        let mut ret = vec![];

        if n_sum < n || n_sum > 6 * n {
            return ret;
        }

        for i in (0..n).rev() {
            let x = (n_sum - 6 * i).max(1);
            n_sum -= x;
            ret.push(x);
        }

        ret
    }
}
```
