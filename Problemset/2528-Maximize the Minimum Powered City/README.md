# 2528. Maximize the Minimum Powered City
You are given a **0-indexed** integer array `stations` of length `n`, where `stations[i]` represents the number of power stations in the <code>i<sup>th</sup></code> city.

Each power station can provide power to every city in a fixed **range**. In other words, if the range is denoted by `r`, then a power station at city `i` can provide power to all cities `j` such that `|i - j| <= r` and `0 <= i, j <= n - 1`.

* Note that `|x|` denotes **absolute** value. For example, `|7 - 5| = 2` and `|3 - 10| = 7`.

The **power** of a city is the total number of power stations it is being provided power from.

The government has sanctioned building `k` more power stations, each of which can be built in any city, and have the same range as the pre-existing ones.

Given the two integers `r` and `k`, return *the **maximum possible minimum power** of a city, if the additional power stations are built optimally*.

**Note** that you can build the `k` power stations in multiple cities.

#### Example 1:
<pre>
<strong>Input:</strong> stations = [1,2,4,5,0], r = 1, k = 2
<strong>Output:</strong> 5
<strong>Explanation:</strong>
One of the optimal ways is to install both the power stations at city 1.
So stations will become [1,4,4,5,0].
- City 0 is provided by 1 + 4 = 5 power stations.
- City 1 is provided by 1 + 4 + 4 = 9 power stations.
- City 2 is provided by 4 + 4 + 5 = 13 power stations.
- City 3 is provided by 5 + 4 = 9 power stations.
- City 4 is provided by 5 + 0 = 5 power stations.
So the minimum power of a city is 5.
Since it is not possible to obtain a larger power, we return 5.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> stations = [4,4,4,4], r = 0, k = 3
<strong>Output:</strong> 4
<strong>Explanation:</strong>
It can be proved that we cannot make the minimum power of a city greater than 4.
</pre>

#### Constraints:
* `n == stations.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>0 <= stations[i] <= 10<sup>5</sup></code>
* `0 <= r <= n - 1`
* <code>0 <= k <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let stations = stations.into_iter().map(|p| p as i64).collect::<Vec<_>>();
        let r = r as usize;
        let k = k as i64;
        let n = stations.len();
        let mut lo = i64::MAX;
        let mut hi = i64::MIN;

        for i in 0..n {
            lo = lo.min(stations[i]);
            hi = hi.max(stations[i] * (r as i64 * 2 + 1) + k);
        }

        while lo < hi {
            let mid = (lo + hi + 1) / 2;
            let mut new_stations = stations.clone();
            let mut power = (0..r).map(|i| stations[i]).sum::<i64>();
            let mut build = 0;

            for i in 0..stations.len() {
                if i > r {
                    power -= new_stations[i - r - 1];
                }
                if i + r < n {
                    power += new_stations[i + r];
                }
                if power < mid {
                    new_stations[(i + r).min(n - 1)] += mid - power;
                    build += mid - power;
                    power = mid;
                }

                if build > k {
                    break;
                }
            }

            if build <= k {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }

        lo
    }
}
```
