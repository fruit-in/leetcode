# 2402. Meeting Rooms III
You are given an integer `n`. There are `n` rooms numbered from `0` to `n - 1`.

You are given a 2D integer array `meetings` where <code>meetings[i] = [start<sub>i</sub>, end<sub>i</sub>]</code> means that a meeting will be held during the **half-closed** time interval <code>[start<sub>i</sub>, end<sub>i</sub>)</code>. All the values of <code>start<sub>i</sub></code> are **unique**.

Meetings are allocated to rooms in the following manner:

1. Each meeting will take place in the unused room with the **lowest** number.
2. If there are no available rooms, the meeting will be delayed until a room becomes free. The delayed meeting should have the **same** duration as the original meeting.
3. When a room becomes unused, meetings that have an earlier original **start** time should be given the room.

Return *the **number** of the room that held the most meetings*. If there are multiple rooms, return *the room with the **lowest** number*.

A **half-closed interval** `[a, b)` is the interval between `a` and `b` **including** `a` and **not including** `b`.

#### Example 1:
<pre>
<strong>Input:</strong> n = 2, meetings = [[0,10],[1,5],[2,7],[3,4]]
<strong>Output:</strong> 0
<strong>Explanation:</strong>
- At time 0, both rooms are not being used. The first meeting starts in room 0.
- At time 1, only room 1 is not being used. The second meeting starts in room 1.
- At time 2, both rooms are being used. The third meeting is delayed.
- At time 3, both rooms are being used. The fourth meeting is delayed.
- At time 5, the meeting in room 1 finishes. The third meeting starts in room 1 for the time period [5,10).
- At time 10, the meetings in both rooms finish. The fourth meeting starts in room 0 for the time period [10,11).
Both rooms 0 and 1 held 2 meetings, so we return 0.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 3, meetings = [[1,20],[2,10],[3,5],[4,9],[6,8]]
<strong>Output:</strong> 1
<strong>Explanation:</strong>
- At time 1, all three rooms are not being used. The first meeting starts in room 0.
- At time 2, rooms 1 and 2 are not being used. The second meeting starts in room 1.
- At time 3, only room 2 is not being used. The third meeting starts in room 2.
- At time 4, all three rooms are being used. The fourth meeting is delayed.
- At time 5, the meeting in room 2 finishes. The fourth meeting starts in room 2 for the time period [5,10).
- At time 6, all three rooms are being used. The fifth meeting is delayed.
- At time 10, the meetings in rooms 1 and 2 finish. The fifth meeting starts in room 1 for the time period [10,12).
Room 0 held 1 meeting while rooms 1 and 2 each held 2 meetings, so we return 1.
</pre>

#### Constraints:
* `1 <= n <= 100`
* <code>1 <= meetings.length <= 10<sup>5</sup></code>
* `meetings[i].length == 2`
* <code>0 <= start<sub>i</sub> < end<sub>i</sub> <= 5 * 10<sup>5</sup></code>
* All the values of <code>start<sub>i</sub></code> are **unique**.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut meetings = meetings;
        let mut free = (-(n - 1)..=0).collect::<BinaryHeap<_>>();
        let mut used = BinaryHeap::<(i64, i32)>::new();
        let mut time = 0_i64;
        let mut count = vec![0; n as usize];

        meetings.sort_unstable();

        for meeting in &meetings {
            let (start, end) = (meeting[0] as i64, meeting[1] as i64);

            if free.is_empty() {
                time = time.max(-used.peek().unwrap().0);
            }
            time = time.max(start);

            while let Some(&(t, r)) = used.peek() {
                if -t <= time {
                    free.push(r);
                    used.pop();
                } else {
                    break;
                }
            }

            used.push((start - end - time, *free.peek().unwrap()));
            count[(-free.pop().unwrap()) as usize] += 1;
        }

        (0..n).max_by_key(|&r| (count[r as usize], -r)).unwrap()
    }
}
```
