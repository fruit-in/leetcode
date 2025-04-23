# 2532. Time to Cross a Bridge
There are `k` workers who want to move `n` boxes from the right (old) warehouse to the left (new) warehouse. You are given the two integers `n` and `k`, and a 2D integer array `time` of size `k x 4` where <code>time[i] = [right<sub>i</sub>, pick<sub>i</sub>, left<sub>i</sub>, put<sub>i</sub>]</code>.

The warehouses are separated by a river and connected by a bridge. Initially, all `k` workers are waiting on the left side of the bridge. To move the boxes, the <code>i<sup>th</sup></code> worker can do the following:
* Cross the bridge to the right side in <code>right<sub>i</sub></code> minutes.
* Pick a box from the right warehouse in <code>pick<sub>i</sub></code> minutes.
* Cross the bridge to the left side in <code>left<sub>i</sub></code> minutes.
* Put the box into the left warehouse in <code>put<sub>i</sub></code> minutes.

The <code>i<sup>th</sup></code> worker is **less efficient** than the <code>j<sup>th</sup></code> worker if either condition is met:
* <code>left<sub>i</sub> + right<sub>i</sub> > left<sub>j</sub> + right<sub>j</sub></code>
* <code>left<sub>i</sub> + right<sub>i</sub> == left<sub>j</sub> + right<sub>j</sub></code> and `i > j`

The following rules regulate the movement of the workers through the bridge:
* Only one worker can use the bridge at a time.
* When the bridge is unused prioritize the **least efficient** worker (who have picked up the box) on the right side to cross. If not, prioritize the **least efficient** worker on the left side to cross.
* If enough workers have already been dispatched from the left side to pick up all the remaining boxes, **no more** workers will be sent from the left side.

Return the **elapsed minutes** at which the last box reaches the **left side of the bridge**.

#### Example 1:
<pre>
<strong>Input:</strong> n = 1, k = 3, time = [[1,1,2,1],[1,1,3,1],[1,1,4,1]]
<strong>Output:</strong> 6
<strong>Explanation:</strong>
From 0 to 1 minutes: worker 2 crosses the bridge to the right.
From 1 to 2 minutes: worker 2 picks up a box from the right warehouse.
From 2 to 6 minutes: worker 2 crosses the bridge to the left.
From 6 to 7 minutes: worker 2 puts a box at the left warehouse.
The whole process ends after 7 minutes. We return 6 because the problem asks for the instance of time at which the last worker reaches the left side of the bridge.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 3, k = 2, time = [[1,5,1,8],[10,10,10,10]]
<strong>Output:</strong> 37
<strong>Explanation:</strong>
<img src="https://assets.leetcode.com/uploads/2024/11/21/378539249-c6ce3c73-40e7-4670-a8b5-7ddb9abede11.png">
The last box reaches the left side at 37 seconds. Notice, how we do not put the last boxes down, as that would take more time, and they are already on the left with the workers.
</pre>

#### Constraints:
* <code>1 <= n, k <= 10<sup>4</sup></code>
* `time.length == k`
* `time[i].length == 4`
* <code>1 <= left<sub>i</sub>, pick<sub>i</sub>, right<sub>i</sub>, put<sub>i</sub> <= 1000</code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_crossing_time(n: i32, k: i32, time: Vec<Vec<i32>>) -> i32 {
        let mut right_workers = BinaryHeap::new();
        let mut pick_workers = BinaryHeap::new();
        let mut left_workers = BinaryHeap::new();
        let mut put_workers = BinaryHeap::new();
        let mut pick = 0;
        let mut put = 0;
        let mut ret = 0;

        for i in 0..time.len() {
            left_workers.push((time[i][2] + time[i][0], i));
        }

        while put < n {
            while pick_workers.peek().unwrap_or(&(Reverse(i32::MAX), 0)).0 .0 <= ret {
                let i = pick_workers.pop().unwrap().1;
                right_workers.push((time[i][2] + time[i][0], i));
            }
            while put < n && put_workers.peek().unwrap_or(&(Reverse(i32::MAX), 0)).0 .0 <= ret {
                let i = put_workers.pop().unwrap().1;
                left_workers.push((time[i][2] + time[i][0], i));
            }

            if let Some((_, i)) = right_workers.pop() {
                ret += time[i][2];
                put_workers.push((Reverse(ret + time[i][3]), i));
                put += 1;
            } else if pick == n || left_workers.is_empty() {
                ret = pick_workers
                    .peek()
                    .unwrap_or(&(Reverse(i32::MAX), 0))
                    .0
                     .0
                    .min(put_workers.peek().unwrap_or(&(Reverse(i32::MAX), 0)).0 .0);
            } else if let Some((_, i)) = left_workers.pop() {
                ret += time[i][0];
                pick_workers.push((Reverse(ret + time[i][1]), i));
                pick += 1;
            }
        }

        ret
    }
}
```
