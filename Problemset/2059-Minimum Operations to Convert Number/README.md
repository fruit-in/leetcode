# 2059. Minimum Operations to Convert Number
You are given a **0-indexed** integer array `nums` containing **distinct** numbers, an integer `start`, and an integer `goal`. There is an integer `x` that is initially set to `start`, and you want to perform operations on `x` such that it is converted to `goal`. You can perform the following operation repeatedly on the number `x`:

If `0 <= x <= 1000`, then for any index `i` in the array (`0 <= i < nums.length`), you can set `x` to any of the following:

* `x + nums[i]`
* `x - nums[i]`
* `x ^ nums[i]` (bitwise-XOR)

Note that you can use each `nums[i]` any number of times in any order. Operations that set `x` to be out of the range `0 <= x <= 1000` are valid, but no more operations can be done afterward.

Return *the **minimum** number of operations needed to convert* `x = start` *into* `goal`, *and* `-1` *if it is not possible*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,4,12], start = 2, goal = 12
<strong>Output:</strong> 2
<strong>Explanation:</strong> We can go from 2 → 14 → 12 with the following 2 operations.
- 2 + 12 = 14
- 14 - 2 = 12
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [3,5,7], start = 0, goal = -4
<strong>Output:</strong> 2
<strong>Explanation:</strong> We can go from 0 → 3 → -4 with the following 2 operations.
- 0 + 3 = 3
- 3 - 7 = -4
Note that the last operation sets x out of the range 0 <= x <= 1000, which is valid.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [2,8,16], start = 0, goal = 1
<strong>Output:</strong> -1
<strong>Explanation:</strong> There is no way to convert 0 into 1.
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* <code>-10<sup>9</sup> <= nums[i], goal <= 10<sup>9</sup></code>
* `0 <= start <= 1000`
* `start != goal`
* All the integers in `nums` are distinct.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>, start: i32, goal: i32) -> i32 {
        let mut unvisited = [true; 1001];
        let mut xs = VecDeque::from([(start, 0)]);

        while let Some((x, count)) = xs.pop_front() {
            for &num in &nums {
                let (a, b, c) = (x + num, x - num, x ^ num);

                if a == goal || b == goal || c == goal {
                    return count + 1;
                }

                if a >= 0 && a <= 1000 && unvisited[a as usize] {
                    unvisited[a as usize] = false;
                    xs.push_back((a, count + 1));
                }
                if b >= 0 && b <= 1000 && unvisited[b as usize] {
                    unvisited[b as usize] = false;
                    xs.push_back((b, count + 1));
                }
                if c >= 0 && c <= 1000 && unvisited[c as usize] {
                    unvisited[c as usize] = false;
                    xs.push_back((c, count + 1));
                }
            }
        }

        -1
    }
}
```
