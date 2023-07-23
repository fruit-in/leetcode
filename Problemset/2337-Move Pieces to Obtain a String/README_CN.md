# 2337. 移动片段得到字符串
给你两个字符串 `start` 和 `target` ，长度均为 `n` 。每个字符串 **仅** 由字符 `'L'`、`'R'` 和 `'_'` 组成，其中：

* 字符 `'L'` 和 `'R'` 表示片段，其中片段 `'L'` 只有在其左侧直接存在一个 **空位** 时才能向 **左** 移动，而片段 `'R'` 只有在其右侧直接存在一个 **空位** 时才能向 **右** 移动。
* 字符 `'_'` 表示可以被 **任意** `'L'` 或 `'R'` 片段占据的空位。

如果在移动字符串 `start` 中的片段任意次之后可以得到字符串 `target` ，返回 `true` ；否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> start = "_L__R__R_", target = "L______RR"
<strong>输出:</strong> true
<strong>解释:</strong> 可以从字符串 start 获得 target ，需要进行下面的移动：
- 将第一个片段向左移动一步，字符串现在变为 "L___R__R_" 。
- 将最后一个片段向右移动一步，字符串现在变为 "L___R___R" 。
- 将第二个片段向右移动散步，字符串现在变为 "L______RR" 。
可以从字符串 start 得到 target ，所以返回 true 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> start = "R_L_", target = "__LR"
<strong>输出:</strong> false
<strong>解释:</strong> 字符串 start 中的 'R' 片段可以向右移动一步得到 "_RL_" 。
但是，在这一步之后，不存在可以移动的片段，所以无法从字符串 start 得到 target 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> start = "_R", target = "R_"
<strong>输出:</strong> false
<strong>解释:</strong> 字符串 start 中的片段只能向右移动，所以无法从字符串 start 得到 target 。
</pre>

#### 提示:
* `n == start.length == target.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* `start` 和 `target` 由字符 `'L'`、`'R'` 和 `'_'` 组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let mut start_pieces = start
            .chars()
            .enumerate()
            .filter(|(_, c)| *c != '_')
            .collect::<Vec<_>>();
        let mut target_pieces = target
            .chars()
            .enumerate()
            .filter(|(_, c)| *c != '_')
            .collect::<Vec<_>>();

        if start_pieces.len() != target_pieces.len() {
            return false;
        }

        for i in 0..start_pieces.len() {
            match (start_pieces[i], target_pieces[i]) {
                ((j, 'L'), (k, 'L')) if j >= k => (),
                ((j, 'R'), (k, 'R')) if j <= k => (),
                _ => return false,
            }
        }

        true
    }
}
```
