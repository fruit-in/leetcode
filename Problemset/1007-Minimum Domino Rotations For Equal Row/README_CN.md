# 1007. 行相等的最少多米诺旋转
在一排多米诺骨牌中，`tops[i]` 和 `bottoms[i]` 分别代表第 `i` 个多米诺骨牌的上半部分和下半部分。（一个多米诺是两个从 1 到 6 的数字同列平铺形成的 —— 该平铺的每一半上都有一个数字。）

我们可以旋转第 `i` 张多米诺，使得 `tops[i]` 和 `bottoms[i]` 的值交换。

返回能使 `tops` 中所有值或者 `bottoms` 中所有值都相同的最小旋转次数。

如果无法做到，返回 `-1`.

#### 示例 1:
![](https://assets.leetcode.com/uploads/2021/05/14/domino.png)
<pre>
<strong>输入:</strong> tops = [2,1,2,4,2,2], bottoms = [5,2,6,2,3,2]
<strong>输出:</strong> 2
<strong>解释:</strong>
图一表示：在我们旋转之前， tops 和 bottoms 给出的多米诺牌。
如果我们旋转第二个和第四个多米诺骨牌，我们可以使上面一行中的每个值都等于 2，如图二所示。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> tops = [3,5,1,2,3], bottoms = [3,6,3,3,4]
<strong>输出:</strong> -1
<strong>解释:</strong>
在这种情况下，不可能旋转多米诺牌使一行的值相等。
</pre>

#### 提示:
* <code>2 <= tops.length <= 2 * 10<sup>4</sup></code>
* `bottoms.length == tops.length`
* `1 <= tops[i], bottoms[i] <= 6`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let mut rotate_top = [0, 1];
        let mut rotate_bottom = [0, 1];

        for i in 1..tops.len() {
            if rotate_top[0] >= 0 && tops[i] != tops[0] && bottoms[i] == tops[0] {
                rotate_top[0] += 1;
            } else if tops[i] != tops[0] {
                rotate_top[0] = -1;
            }
            if rotate_top[1] >= 0 && bottoms[i] != tops[0] && tops[i] == tops[0] {
                rotate_top[1] += 1;
            } else if bottoms[i] != tops[0] {
                rotate_top[1] = -1;
            }
            if rotate_bottom[0] >= 0 && bottoms[i] != bottoms[0] && tops[i] == bottoms[0] {
                rotate_bottom[0] += 1;
            } else if bottoms[i] != bottoms[0] {
                rotate_bottom[0] = -1;
            }
            if rotate_bottom[1] >= 0 && tops[i] != bottoms[0] && bottoms[i] == bottoms[0] {
                rotate_bottom[1] += 1;
            } else if tops[i] != bottoms[0] {
                rotate_bottom[1] = -1;
            }
        }

        *rotate_top
            .iter()
            .chain(rotate_bottom.iter())
            .filter(|&&x| x >= 0)
            .min()
            .unwrap_or(&-1)
    }
}
```
