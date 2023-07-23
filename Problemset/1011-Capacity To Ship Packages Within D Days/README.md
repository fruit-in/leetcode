# 1011. Capacity To Ship Packages Within D Days
A conveyor belt has packages that must be shipped from one port to another within `days` days.

The <code>i<sup>th</sup></code> package on the conveyor belt has a weight of `weights[i]`. Each day, we load the ship with packages on the conveyor belt (in the order given by `weights`). We may not load more weight than the maximum weight capacity of the ship.

Return the least weight capacity of the ship that will result in all the packages on the conveyor belt being shipped within `days` days.

#### Example 1:
<pre>
<strong>Input:</strong> weights = [1,2,3,4,5,6,7,8,9,10], days = 5
<strong>Output:</strong> 15
<strong>Explanation:</strong> A ship capacity of 15 is the minimum to ship all the packages in 5 days like this:
1st day: 1, 2, 3, 4, 5
2nd day: 6, 7
3rd day: 8
4th day: 9
5th day: 10

Note that the cargo must be shipped in the order given, so using a ship of capacity 14 and splitting the packages into parts like (2, 3, 4, 5), (1, 6, 7), (8), (9), (10) is not allowed.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> weights = [3,2,2,4,1,4], days = 3
<strong>Output:</strong> 6
<strong>Explanation:</strong> A ship capacity of 6 is the minimum to ship all the packages in 3 days like this:
1st day: 3, 2
2nd day: 2, 4
3rd day: 1, 4
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> weights = [1,2,3,1,1], days = 4
<strong>Output:</strong> 3
<strong>Explanation:</strong>
1st day: 1
2nd day: 2
3rd day: 3
4th day: 1, 1
</pre>

#### Constraints:
* <code>1 <= days <= weights.length <= 5 * 10<sup>4</sup></code>
* `1 <= weights[i] <= 500`

## Solutions (Rust)

### 1. Binary Search
```Rust
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let max_weight = *weights.iter().max().unwrap();
        let sum_weight = weights.iter().sum::<i32>();
        let capacities = (max_weight..=sum_weight).collect::<Vec<_>>();

        match capacities.binary_search_by_key(&false, |&c| Self::shipped_in_time(&weights, c, days))
        {
            Ok(c) => c as i32 + 1 + max_weight,
            Err(c) => c as i32 + max_weight,
        }
    }

    fn shipped_in_time(weights: &[i32], capacity: i32, days: i32) -> bool {
        let mut remain = 0;
        let mut spend = 0;

        for &w in weights {
            if remain < w {
                remain = capacity;
                spend += 1;
            }
            remain -= w;
        }

        spend <= days
    }
}
```
