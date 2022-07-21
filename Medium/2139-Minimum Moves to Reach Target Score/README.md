# 2139. Minimum Moves to Reach Target Score
You are playing a game with integers. You start with the integer `1` and you want to reach the integer `target`.

In one move, you can either:
* **Increment** the current integer by one (i.e., `x = x + 1`).
* **Double** the current integer (i.e., `x = 2 * x`).

You can use the **increment** operation **any** number of times, however, you can only use the **double** operation **at most** `maxDoubles` times.

Given the two integers `target` and `maxDoubles`, return *the minimum number of moves needed to reach* `target` *starting with* `1`.

#### Example 1:
<pre>
<strong>Input:</strong> target = 5, maxDoubles = 0
<strong>Output:</strong> 4
<strong>Explanation:</strong> Keep incrementing by 1 until you reach target.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> target = 19, maxDoubles = 2
<strong>Output:</strong> 7
<strong>Explanation:</strong> Initially, x = 1
Increment 3 times so x = 4
Double once so x = 8
Increment once so x = 9
Double again so x = 18
Increment once so x = 19
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> target = 10, maxDoubles = 4
<strong>Output:</strong> 4
<strong>Explanation:</strong> Initially, x = 1
Increment once so x = 2
Double once so x = 4
Increment once so x = 5
Double again so x = 10
</pre>

#### Constraints:
* <code>1 <= target <= 10<sup>9</sup></code>
* `0 <= maxDoubles <= 100`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_moves(target: i32, max_doubles: i32) -> i32 {
        let mut target = target;
        let mut max_doubles = max_doubles;
        let mut ret = 0;

        while target > 1 {
            if max_doubles == 0 {
                return ret + target - 1;
            } else if target % 2 == 1 {
                target -= 1;
            } else {
                target /= 2;
                max_doubles -= 1;
            }

            ret += 1;
        }

        ret
    }
}
```
