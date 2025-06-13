# 1345. Jump Game IV
Given an array of integers `arr`, you are initially positioned at the first index of the array.

In one step you can jump from index `i` to index:
* `i + 1` where: `i + 1 < arr.length`.
* `i - 1` where: `i - 1 >= 0`.
* `j` where: `arr[i] == arr[j]` and `i != j`.

Return *the minimum number of steps* to reach the **last index** of the array.

Notice that you can not jump outside of the array at any time.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [100,-23,-23,404,100,23,23,23,3,404]
<strong>Output:</strong> 3
<strong>Explanation:</strong> You need three jumps from index 0 --> 4 --> 3 --> 9. Note that index 9 is the last index of the array.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [7]
<strong>Output:</strong> 0
<strong>Explanation:</strong> Start index is the last index. You do not need to jump.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [7,6,9,6,9,6,9,7]
<strong>Output:</strong> 1
<strong>Explanation:</strong> You can jump directly from index 0 to index 7 which is last index of the array.
</pre>

#### Constraints:
* <code>1 <= arr.length <= 5 * 10<sup>4</sup></code>
* <code>-10<sup>8</sup> <= arr[i] <= 10<sup>8</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut js = HashMap::new();
        let mut visited = HashSet::from([0]);
        let mut deque = VecDeque::from([(0, 0)]);

        for i in 0..arr.len() {
            js.entry(arr[i]).or_insert(vec![]).push(i);
        }

        while let Some((i, steps)) = deque.pop_front() {
            if i == arr.len() - 1 {
                return steps;
            }

            if i > 0 && !visited.contains(&(i - 1)) {
                visited.insert(i - 1);
                deque.push_back((i - 1, steps + 1));
            }
            if i < arr.len() - 1 && !visited.contains(&(i + 1)) {
                visited.insert(i + 1);
                deque.push_back((i + 1, steps + 1));
            }
            for j in js.remove(&arr[i]).unwrap_or(vec![]) {
                if !visited.contains(&j) {
                    visited.insert(j);
                    deque.push_back((j, steps + 1));
                }
            }
        }

        unreachable!()
    }
}
```
