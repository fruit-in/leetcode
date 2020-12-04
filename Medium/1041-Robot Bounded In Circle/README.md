# 1041. Robot Bounded In Circle
On an infinite plane, a robot initially stands at `(0, 0)` and faces north.  The robot can receive one of three instructions:
* `"G"`: go straight 1 unit;
* `"L"`: turn 90 degrees to the left;
* `"R"`: turn 90 degress to the right.

The robot performs the `instructions` given in order, and repeats them forever.

Return `true` if and only if there exists a circle in the plane such that the robot never leaves the circle.

#### Example 1:
<pre>
<strong>Input:</strong> "GGLLGG"
<strong>Output:</strong> true
<strong>Explanation:</strong>
The robot moves from (0,0) to (0,2), turns 180 degrees, and then returns to (0,0).
When repeating these instructions, the robot remains in the circle of radius 2 centered at the origin.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "GG"
<strong>Output:</strong> false
<strong>Explanation:</strong>
The robot moves north indefinitely.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> "GL"
<strong>Output:</strong> true
<strong>Explanation:</strong>
The robot moves from (0, 0) -> (0, 1) -> (-1, 1) -> (-1, 0) -> (0, 0) -> ...
</pre>

#### Note:
1. `1 <= instructions.length <= 100`
2. `instructions[i]` is in `{'G', 'L', 'R'}`

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut position = (0, 0);
        let mut direction = 0;

        for ins in instructions.chars() {
            match (ins, direction % 4) {
                ('G', 0) => position.1 += 1,
                ('G', 1) => position.0 += 1,
                ('G', 2) => position.1 -= 1,
                ('G', 3) => position.0 -= 1,
                ('L', _) => direction += 3,
                _ => direction += 1,
            }
        }

        position == (0, 0) || direction % 4 != 0
    }
}
```
