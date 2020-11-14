# 1109. Corporate Flight Bookings
There are `n` flights, and they are labeled from `1` to `n`.

We have a list of flight bookings.  The `i`-th booking `bookings[i] = [i, j, k]` means that we booked `k` seats from flights labeled `i` to `j` inclusive.

Return an array `answer` of length `n`, representing the number of seats booked on each flight in order of their label.

#### Example 1:
<pre>
<b>Input:</b> bookings = [[1,2,10],[2,3,20],[2,5,25]], n = 5
<b>Output:</b> [10,55,45,25,25]
</pre>

#### Constraints:
* `1 <= bookings.length <= 20000`
* `1 <= bookings[i][0] <= bookings[i][1] <= n <= 20000`
* `1 <= bookings[i][2] <= 10000`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut ret = vec![0; n as usize];

        for booking in bookings {
            if booking[0] > 1 {
                ret[booking[0] as usize - 2] -= booking[2];
            }
            ret[booking[1] as usize - 1] += booking[2];
        }

        for i in (0..(n as usize - 1)).rev() {
            ret[i] += ret[i + 1];
        }

        ret
    }
}
```
