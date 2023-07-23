# 1942. The Number of the Smallest Unoccupied Chair
There is a party where `n` friends numbered from `0` to `n - 1` are attending. There is an **infinite** number of chairs in this party that are numbered from `0` to `infinity`. When a friend arrives at the party, they sit on the unoccupied chair with the **smallest number**.

* For example, if chairs `0`, `1`, and `5` are occupied when a friend comes, they will sit on chair number `2`.

When a friend leaves the party, their chair becomes unoccupied at the moment they leave. If another friend arrives at that same moment, they can sit in that chair.

You are given a **0-indexed** 2D integer array `times` where <code>times[i] = [arrival<sub>i</sub>, leaving<sub>i</sub>]</code>, indicating the arrival and leaving times of the <code>i<sup>th</sup></code> friend respectively, and an integer `targetFriend`. All arrival times are **distinct**.

Return *the **chair number** that the friend numbered* `targetFriend` *will sit on*.

#### Example 1:
<pre>
<strong>Input:</strong> times = [[1,4],[2,3],[4,6]], targetFriend = 1
<strong>Output:</strong> 1
<strong>Explanation:</strong>
- Friend 0 arrives at time 1 and sits on chair 0.
- Friend 1 arrives at time 2 and sits on chair 1.
- Friend 1 leaves at time 3 and chair 1 becomes empty.
- Friend 0 leaves at time 4 and chair 0 becomes empty.
- Friend 2 arrives at time 4 and sits on chair 0.
Since friend 1 sat on chair 1, we return 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> times = [[3,10],[1,5],[2,6]], targetFriend = 0
<strong>Output:</strong> 2
<strong>Explanation:</strong>
- Friend 1 arrives at time 1 and sits on chair 0.
- Friend 2 arrives at time 2 and sits on chair 1.
- Friend 0 arrives at time 3 and sits on chair 2.
- Friend 1 leaves at time 5 and chair 0 becomes empty.
- Friend 2 leaves at time 6 and chair 1 becomes empty.
- Friend 0 leaves at time 10 and chair 2 becomes empty.
Since friend 0 sat on chair 2, we return 2.
</pre>

#### Constraints:
* `n == times.length`
* <code>2 <= n <= 10<sup>4</sup></code>
* `times[i].length == 2`
* <code>1 <= arrival<sub>i</sub> < leaving<sub>i</sub> <= 10<sup>5</sup></code>
* `0 <= targetFriend <= n - 1`
* Each <code>arrival<sub>i</sub></code> time is **distinct**.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let target_arrival = times[target_friend as usize][0];
        let mut chair_inf = 0;
        let mut chair_heap = BinaryHeap::new();
        let mut leaving_heap = BinaryHeap::new();
        let mut times = times;
        times.sort_unstable();

        for time in &times {
            while -leaving_heap.peek().unwrap_or(&(-100001, 0)).0 <= time[0] {
                chair_heap.push(leaving_heap.pop().unwrap().1);
            }

            match chair_heap.pop() {
                Some(chair) if time[0] == target_arrival => return -chair,
                Some(chair) => leaving_heap.push((-time[1], chair)),
                None if time[0] == target_arrival => return chair_inf,
                None => {
                    leaving_heap.push((-time[1], -chair_inf));
                    chair_inf += 1;
                }
            }
        }

        unreachable!()
    }
}
```
