# 1386. Cinema Seat Allocation
![](https://assets.leetcode.com/uploads/2020/02/14/cinema_seats_1.png)

A cinema has `n` rows of seats, numbered from 1 to `n` and there are ten seats in each row, labelled from 1 to 10 as shown in the figure above.

Given the array `reservedSeats` containing the numbers of seats already reserved, for example, `reservedSeats[i] = [3,8]` means the seat located in row **3** and labelled with **8** is already reserved.

*Return the maximum number of four-person groups you can assign on the cinema seats*. A four-person group occupies four adjacent seats **in one single row**. Seats across an aisle (such as [3,3] and [3,4]) are not considered to be adjacent, but there is an exceptional case on which an aisle split a four-person group, in that case, the aisle split a four-person group in the middle, which means to have two people on each side.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/02/14/cinema_seats_3.png)
<pre>
<strong>Input:</strong> n = 3, reservedSeats = [[1,2],[1,3],[1,8],[2,6],[3,1],[3,10]]
<strong>Output:</strong> 4
<strong>Explanation:</strong> The figure above shows the optimal allocation for four groups, where seats mark with blue are already reserved and contiguous seats mark with orange are for one group.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 2, reservedSeats = [[2,1],[1,8],[2,6]]
<strong>Output:</strong> 2
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 4, reservedSeats = [[4,3],[1,4],[4,6],[1,7]]
<strong>Output:</strong> 4
</pre>

#### Constraints:
* `1 <= n <= 10^9`
* `1 <= reservedSeats.length <= min(10*n, 10^4)`
* `reservedSeats[i].length == 2`
* `1 <= reservedSeats[i][0] <= n`
* `1 <= reservedSeats[i][1] <= 10`
* All `reservedSeats[i]` are distinct.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut seats_map = HashMap::new();
        let mut ret = 0;

        for seat in &reserved_seats {
            seats_map
                .entry(seat[0])
                .and_modify(|x| *x |= 1 << seat[1])
                .or_insert(1 << seat[1]);
        }

        for row in seats_map.values() {
            if row & 0x3fc == 0 {
                ret += 2;
            } else if row & 0x3c == 0 {
                ret += 1;
            } else if row & 0xf0 == 0 {
                ret += 1;
            } else if row & 0x3c0 == 0 {
                ret += 1;
            }
        }

        ret += (n - seats_map.len() as i32) * 2;

        ret
    }
}
```
