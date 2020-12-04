# 1041. 困于环中的机器人
在无限的平面上，机器人最初位于 `(0, 0)` 处，面朝北方。机器人可以接受下列三条指令之一：
* `"G"`：直走 1 个单位
* `"L"`：左转 90 度
* `"R"`：右转 90 度

机器人按顺序执行指令 `instructions`，并一直重复它们。

只有在平面中存在环使得机器人永远无法离开时，返回 `true`。否则，返回 `false`。

#### 示例 1:
<pre>
<strong>输入:</strong> "GGLLGG"
<strong>输出:</strong> true
<strong>解释:</strong>
机器人从 (0,0) 移动到 (0,2)，转 180 度，然后回到 (0,0)。
重复这些指令，机器人将保持在以原点为中心，2 为半径的环中进行移动。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "GG"
<strong>输出:</strong> false
<strong>解释:</strong>
机器人无限向北移动。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> "GL"
<strong>输出:</strong> true
<strong>解释:</strong>
机器人按 (0, 0) -> (0, 1) -> (-1, 1) -> (-1, 0) -> (0, 0) -> ... 进行移动。
</pre>

#### 提示:
1. `1 <= instructions.length <= 100`
2. `instructions[i]` 在 `{'G', 'L', 'R'}` 中

## 题解 (Rust)

### 1. 题解
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
