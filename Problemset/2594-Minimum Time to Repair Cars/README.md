# 2594. Minimum Time to Repair Cars
You are given an integer array `ranks` representing the **ranks** of some mechanics. <code>ranks<sub>i</sub></code> is the rank of the i<sup>th<;sup> mechanic. A mechanic with a rank `r` can repair n cars in <code>r * n<sup>2</sup></code> minutes.

You are also given an integer `cars` representing the total number of cars waiting in the garage to be repaired.

Return *the **minimum** time taken to repair all the cars*.

**Note:** All the mechanics can repair the cars simultaneously.

#### Example 1:
<pre>
<strong>Input:</strong> ranks = [4,2,3,1], cars = 10
<strong>Output:</strong> 16
<strong>Explanation:</strong>
- The first mechanic will repair two cars. The time required is 4 * 2 * 2 = 16 minutes.
- The second mechanic will repair two cars. The time required is 2 * 2 * 2 = 8 minutes.
- The third mechanic will repair two cars. The time required is 3 * 2 * 2 = 12 minutes.
- The fourth mechanic will repair four cars. The time required is 1 * 4 * 4 = 16 minutes.
It can be proved that the cars cannot be repaired in less than 16 minutes.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> ranks = [5,1,8], cars = 6
<strong>Output:</strong> 16
<strong>Explanation:</strong>
- The first mechanic will repair one car. The time required is 5 * 1 * 1 = 5 minutes.
- The second mechanic will repair four cars. The time required is 1 * 4 * 4 = 16 minutes.
- The third mechanic will repair one car. The time required is 8 * 1 * 1 = 8 minutes.
It can be proved that the cars cannot be repaired in less than 16 minutes.
</pre>

#### Constraints:
* <code>1 <= ranks.length <= 10<sup>5</sup></code>
* `1 <= ranks[i] <= 100`
* <code>1 <= cars <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let mut heap = ranks
            .iter()
            .map(|&r| (-r as i64, r as i64, 0))
            .collect::<BinaryHeap<_>>();

        for _ in 0..cars {
            let mut time_rank_cars = heap.peek_mut().unwrap();
            time_rank_cars.2 += 1;
            time_rank_cars.0 = -time_rank_cars.1 * (time_rank_cars.2 + 1) * (time_rank_cars.2 + 1);
        }

        heap.iter().map(|&(t, r, n)| r * n * n).max().unwrap()
    }
}
```
