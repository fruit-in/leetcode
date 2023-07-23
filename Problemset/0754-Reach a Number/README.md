# 754. Reach a Number
You are standing at position <code>0</code> on an infinite number line. There is a goal at position <code>target</code>.

On each move, you can either go left or right. During the *n*-th move (starting from 1), you take *n* steps.

Return the minimum number of steps required to reach the destination.

#### Example 1:
<pre>
<strong>Input:</strong> target = 3
<strong>Output:</strong> 2
<strong>Explanation:</strong>
On the first move we step from 0 to 1.
On the second step we step from 1 to 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> target = 2
<strong>Output:</strong> 3
<strong>Explanation:</strong>
On the first move we step from 0 to 1.
On the second move we step  from 1 to -1.
On the third move we step from -1 to 2.
</pre>

#### Note:
* <code>target</code> will be a non-zero integer in the range <code>[-10<sup>9</sup>, 10<sup>9</sup>]</code>.

## Solutions (Rust)

### 1. Mathematical
```Rust
impl Solution {
    pub fn reach_number(target: i32) -> i32 {
        let mut target = target;
        let mut position = 0;
        let mut step = 0;
        if target < 0 {
            target = -target;
        }
        while position < target {
            step += 1;
            position += step;
        }
        if (position - target) % 2 == 0 {
            step
        } else if step % 2 == 0 {
            step + 1
        } else {
            step + 2
        }
    }
}
```
