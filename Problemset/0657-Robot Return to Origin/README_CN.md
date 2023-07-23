# 657. 机器人能否返回原点
在二维平面上，有一个机器人从原点 (0, 0) 开始。给出它的移动顺序，判断这个机器人在完成移动后是否在 **(0, 0) 处结束**。

移动顺序由字符串表示。字符 move[i] 表示其第 i 次移动。机器人的有效动作有 ```R```（右），```L```（左），```U```（上）和 ```D```（下）。如果机器人在完成所有动作后返回原点，则返回 true。否则，返回 false。

**注意:** 机器人“面朝”的方向无关紧要。 “R” 将始终使机器人向右移动一次，“L” 将始终向左移动等。此外，假设每次移动机器人的移动幅度相同。

#### 示例 1:
<pre>
<strong>输入:</strong> "UD"
<strong>输出:</strong> true
<strong>解释:</strong> 机器人向上移动一次，然后向下移动一次。所有动作都具有相同的幅度，因此它最终回到它开始的原点。因此，我们返回 true。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "LL"
<strong>输出:</strong> false
<strong>解释:</strong> 机器人向左移动两次。它最终位于原点的左侧，距原点有两次 “移动” 的距离。我们返回 false，因为它在移动结束时没有返回原点。
</pre>

## 题解 (Python)

### 1. 模拟
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

### 2. 计数
```Python3
class Solution:
    def judgeCircle(self, moves: str) -> bool:
        return moves.count('R') == moves.count('L') \
               and moves.count('U') == moves.count('D')
```

## 题解 (Rust)

### 1. 模拟
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

### 2. 计数
```Rust
impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        moves.matches('R').count() == moves.matches('L').count() &&
        moves.matches('U').count() == moves.matches('D').count()
    }
}
```
