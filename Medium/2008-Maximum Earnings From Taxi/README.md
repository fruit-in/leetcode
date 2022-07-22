# 2008. Maximum Earnings From Taxi
There are `n` points on a road you are driving your taxi on. The `n` points on the road are labeled from `1` to `n` in the direction you are going, and you want to drive from point `1` to point `n` to make money by picking up passengers. You cannot change the direction of the taxi.

The passengers are represented by a **0-indexed** 2D integer array `rides`, where `rides[i] = [starti, endi, tipi]` denotes the `ith` passenger requesting a ride from point `starti` to point `endi` who is willing to give a `tipi` dollar tip.

For **each** passenger `i` you pick up, you **earn** `endi - starti + tipi` dollars. You may only drive **at most one** passenger at a time.

Given `n` and `rides`, return *the **maximum** number of dollars you can earn by picking up the passengers optimally*.

**Note:** You may drop off a passenger and pick up a different passenger at the same point.

#### Example 1:
<pre>
<strong>Input:</strong> n = 5, rides = [[2,5,4],[1,5,1]]
<strong>Output:</strong> 7
<strong>Explanation:</strong> We can pick up passenger 0 to earn 5 - 2 + 4 = 7 dollars.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 20, rides = [[1,6,1],[3,10,2],[10,12,3],[11,12,2],[12,15,2],[13,18,1]]
<strong>Output:</strong> 20
<strong>Explanation:</strong> We will pick up the following passengers:
- Drive passenger 1 from point 3 to point 10 for a profit of 10 - 3 + 2 = 9 dollars.
- Drive passenger 2 from point 10 to point 12 for a profit of 12 - 10 + 3 = 5 dollars.
- Drive passenger 5 from point 13 to point 18 for a profit of 18 - 13 + 1 = 6 dollars.
We earn 9 + 5 + 6 = 20 dollars in total.
</pre>

#### Constraints:
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= rides.length <= 3 * 10<sup>4</sup></code>
* `rides[i].length == 3`
* <code>1 <= start<sub>i</sub> < end<sub>i</sub> <= n</code>
* <code>1 <= tip<sub>i</sub> <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_taxi_earnings(n: i32, rides: Vec<Vec<i32>>) -> i64 {
        let mut rides = rides;
        let mut i = 1;
        let mut dp = vec![0; n as usize + 1];
        rides.sort_unstable();

        for ride in rides {
            let (start, end, tip) = (ride[0], ride[1], ride[2]);

            for j in i..=start as usize {
                dp[j] = dp[j].max(dp[j - 1]);
            }

            i = start as usize + 1;
            dp[end as usize] =
                dp[end as usize].max(dp[start as usize] + (end - start + tip) as i64);
        }

        for j in i..=n as usize {
            dp[j] = dp[j].max(dp[j - 1]);
        }

        dp[n as usize]
    }
}
```
