# 2332. The Latest Time to Catch a Bus
You are given a **0-indexed** integer array `buses` of length `n`, where `buses[i]` represents the departure time of the <code>i<sup>th</sup></code> bus. You are also given a **0-indexed** integer array `passengers` of length `m`, where `passengers[j]` represents the arrival time of the <code>j<sup>th</sup></code> passenger. All bus departure times are unique. All passenger arrival times are unique.

You are given an integer `capacity`, which represents the **maximum** number of passengers that can get on each bus.

When a passenger arrives, they will wait in line for the next available bus. You can get on a bus that departs at `x` minutes if you arrive at `y` minutes where `y <= x`, and the bus is not full. Passengers with the **earliest** arrival times get on the bus first.

More formally when a bus arrives, either:
* If `capacity` or fewer passengers are waiting for a bus, they will **all** get on the bus, or
* The `capacity` passengers with the **earliest** arrival times will get on the bus.

Return *the latest time you may arrive at the bus station to catch a bus*. You **cannot** arrive at the same time as another passenger.

**Note:** The arrays `buses` and `passengers` are not necessarily sorted.

#### Example 1:
<pre>
<strong>Input:</strong> buses = [10,20], passengers = [2,17,18,19], capacity = 2
<strong>Output:</strong> 16
<strong>Explanation:</strong> Suppose you arrive at time 16.
At time 10, the first bus departs with the 0th passenger.
At time 20, the second bus departs with you and the 1st passenger.
Note that you may not arrive at the same time as another passenger, which is why you must arrive before the 1st passenger to catch the bus.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> buses = [20,30,10], passengers = [19,13,26,4,25,11,21], capacity = 2
<strong>Output:</strong> 20
<strong>Explanation:</strong> Suppose you arrive at time 20.
At time 10, the first bus departs with the 3rd passenger.
At time 20, the second bus departs with the 5th and 1st passengers.
At time 30, the third bus departs with the 0th passenger and you.
Notice if you had arrived any later, then the 6th passenger would have taken your seat on the third bus.
</pre>

#### Constraints:
* `n == buses.length`
* `m == passengers.length`
* <code>1 <= n, m, capacity <= 10<sup>5</sup></code>
* <code>2 <= buses[i], passengers[i] <= 10<sup>9</sup></code>
* Each element in `buses` is **unique**.
* Each element in `passengers` is **unique**.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn latest_time_catch_the_bus(
        mut buses: Vec<i32>,
        mut passengers: Vec<i32>,
        capacity: i32,
    ) -> i32 {
        let passengers_set = passengers.clone().into_iter().collect::<HashSet<_>>();
        let mut i = 0;
        let mut ret = 1;

        buses.sort_unstable();
        passengers.sort_unstable();

        for j in 0..buses.len() {
            let mut count = 0;
            let mut time = buses[j];

            while i < passengers.len() && passengers[i] <= buses[j] && count < capacity {
                count += 1;
                i += 1;
            }

            if count == capacity {
                time = passengers[i - 1];
            }

            while passengers_set.contains(&time) {
                time -= 1;
            }

            ret = ret.max(time);
        }

        ret
    }
}
```
