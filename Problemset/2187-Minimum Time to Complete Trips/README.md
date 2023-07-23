# 2187. Minimum Time to Complete Trips
You are given an array `time` where `time[i]` denotes the time taken by the <code>i<sup>th</sup></code> bus to complete **one trip**.

Each bus can make multiple trips **successively**; that is, the next trip can start **immediately after** completing the current trip. Also, each bus operates **independently**; that is, the trips of one bus do not influence the trips of any other bus.

You are also given an integer `totalTrips`, which denotes the number of trips all buses should make **in total**. Return *the **minimum time** required for all buses to complete **at least*** `totalTrips` *trips*.

#### Example 1:
<pre>
<strong>Input:</strong> time = [1,2,3], totalTrips = 5
<strong>Output:</strong> 3
<strong>Explanation:</strong>
- At time t = 1, the number of trips completed by each bus are [1,0,0].
  The total number of trips completed is 1 + 0 + 0 = 1.
- At time t = 2, the number of trips completed by each bus are [2,1,0].
  The total number of trips completed is 2 + 1 + 0 = 3.
- At time t = 3, the number of trips completed by each bus are [3,1,1].
  The total number of trips completed is 3 + 1 + 1 = 5.
So the minimum time needed for all buses to complete at least 5 trips is 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> time = [2], totalTrips = 1
<strong>Output:</strong> 2
<strong>Explanation:</strong>
There is only one bus, and it will complete its first trip at t = 2.
So the minimum time needed to complete 1 trip is 2.
</pre>

#### Constraints:
* <code>1 <= time.length <= 10<sup>5</sup></code>
* <code>1 <= time[i], totalTrips <= 10<sup>7</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let total_trips = total_trips as i64;
        let mut l = 1_i64;
        let mut r = total_trips * *time.iter().min().unwrap() as i64;

        while l < r {
            let m = (l + r) / 2;
            let trips = time.iter().map(|&t| m / t as i64).sum::<i64>();

            if trips < total_trips {
                l = m + 1;
            } else {
                r = m;
            }
        }

        r
    }
}
```
