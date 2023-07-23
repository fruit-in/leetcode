# 657. Robot Return to Origin
There is a robot starting at position (0, 0), the origin, on a 2D plane. Given a sequence of its moves, judge if this robot **ends up at (0, 0)** after it completes its moves.

The move sequence is represented by a string, and the character moves[i] represents its ith move. Valid moves are R (right), L (left), U (up), and D (down). If the robot returns to the origin after it finishes all of its moves, return true. Otherwise, return false.

**Note:** The way that the robot is "facing" is irrelevant. "R" will always make the robot move to the right once, "L" will always make it move left, etc. Also, assume that the magnitude of the robot's movement is the same for each move.

#### Example 1:
<pre>
<strong>Input:</strong> "UD"
<strong>Output:</strong> true
<strong>Explanation:</strong> The robot moves up once, and then down once. All moves have the same magnitude, so it ended up
at the origin where it started. Therefore, we return true.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "LL"
<strong>Output:</strong> false
<strong>Explanation:</strong> The robot moves left twice. It ends up two "moves" to the left of the origin. We return false
because it is not at the origin at the end of its moves.
</pre>

## Solutions (Python)

### 1. Simulation
```Python3
class Solution:
    def judgeCircle(self, moves: str) -> bool:
        x, y = 0, 0
        for c in moves:
            if c == 'R':
                x += 1
            elif c == 'L':
                x -= 1
            elif c == 'U':
                y += 1
            elif c == 'D':
                y -= 1
        return x == 0 and y == 0
```

### 2. Count
```Python3
class Solution:
    def judgeCircle(self, moves: str) -> bool:
        return moves.count('R') == moves.count('L') \
               and moves.count('U') == moves.count('D')
```

## Solutions (Rust)

### 1. Simulation
```Rust
impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut x = 0;
        let mut y = 0;
        for ch in moves.chars() {
            match ch {
                'R' => x += 1,
                'L' => x -= 1,
                'U' => y += 1,
                'D' => y -= 1,
                _ => {},
            }
        }
        x == 0 && y == 0
    }
}
```

### 2. Count
```Rust
impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        moves.matches('R').count() == moves.matches('L').count() &&
        moves.matches('U').count() == moves.matches('D').count()
    }
}
```
