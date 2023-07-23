# 1866. Number of Ways to Rearrange Sticks With K Sticks Visible
There are `n` uniquely-sized sticks whose lengths are integers from `1` to `n`. You want to arrange the sticks such that **exactly** `k` sticks are **visible** from the left. A stick is **visible** from the left if there are no **longer** sticks to the **left** of it.
* For example, if the sticks are arranged `[1,3,2,5,4]`, then the sticks with lengths `1`, `3`, and `5` are visible from the left.

Given `n` and `k`, return *the **number** of such arrangements*. Since the answer may be large, return it modulo <code>10<sup>9</sup> + 7</code>.

#### Example 1:
<pre>
<strong>Input:</strong> n = 3, k = 2
<strong>Output:</strong> 3
<strong>Explanation:</strong> [1,3,2], [2,3,1], and [2,1,3] are the only arrangements such that exactly 2 sticks are visible.
The visible sticks are underlined.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 5, k = 5
<strong>Output:</strong> 1
<strong>Explanation:</strong> [1,2,3,4,5] is the only arrangement such that all 5 sticks are visible.
The visible sticks are underlined.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 20, k = 11
<strong>Output:</strong> 647427950
<strong>Explanation:</strong> There are 647427950 (mod 10<sup>9</sup> + 7) ways to rearrange the sticks such that exactly 11 sticks are visible.
</pre>

#### Constraints:
* `1 <= n <= 1000`
* `1 <= k <= n`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn rearrange_sticks(n: i32, k: i32) -> i32 {
        let mut dp0 = vec![0; n as usize + 1];

        for i in 1..=k as usize {
            let mut dp1 = vec![0; n as usize + 1];
            dp1[i] = 1;

            for j in i + 1..=n as usize {
                dp1[j] = (dp1[j - 1] as i64 * (j as i64 - 1) % 1_000_000_007) as i32
                    + dp0[j - 1] % 1_000_000_007;
            }

            dp0 = dp1;
        }

        dp0[n as usize] % 1_000_000_007
    }
}
```
