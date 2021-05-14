# 1705. Maximum Number of Eaten Apples
There is a special kind of apple tree that grows apples every day for `n` days. On the <code>i<sup>th</sup></code> day, the tree grows `apples[i]` apples that will rot after `days[i]` days, that is on day `i + days[i]` the apples will be rotten and cannot be eaten. On some days, the apple tree does not grow any apples, which are denoted by `apples[i] == 0` and `days[i] == 0`.

You decided to eat **at most** one apple a day (to keep the doctors away). Note that you can keep eating after the first `n` days.

Given two integer arrays `days` and `apples` of length `n`, return *the maximum number of apples you can eat*.

#### Example 1:
<pre>
<strong>Input:</strong> apples = [1,2,3,5,2], days = [3,2,1,4,2]
<strong>Output:</strong> 7
<strong>Explanation:</strong> You can eat 7 apples:
- On the first day, you eat an apple that grew on the first day.
- On the second day, you eat an apple that grew on the second day.
- On the third day, you eat an apple that grew on the second day. After this day, the apples that grew on the third day rot.
- On the fourth to the seventh days, you eat apples that grew on the fourth day.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> apples = [3,0,0,0,0,2], days = [3,0,0,0,0,2]
<strong>Output:</strong> 5
<strong>Explanation:</strong> You can eat 5 apples:
- On the first to the third day you eat apples that grew on the first day.
- Do nothing on the fouth and fifth days.
- On the sixth and seventh days you eat apples that grew on the sixth day.
</pre>

#### Constraints:
* `apples.length == n`
* `days.length == n`
* <code>1 <= n <= 2 * 10<sup>4</sup></code>
* <code>0 <= apples[i], days[i] <= 2 * 10<sup>4</sup></code>
* `days[i] = 0` if and only if `apples[i] = 0`.

## Solutions (Rust)

### 1. Heap
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut i = 0;
        let mut max_day = days.len() as i32;
        let mut ret = 0;

        while i <= max_day {
            if (i as usize) < days.len() {
                heap.push((-i - days[i as usize], apples[i as usize]));
                max_day = max_day.max(i + days[i as usize]);
            }

            while let Some((day, apple)) = heap.pop() {
                if -day > i && apple > 0 {
                    heap.push((day, apple - 1));
                    ret += 1;
                    break;
                }
            }

            i += 1;
        }

        ret
    }
}
```
