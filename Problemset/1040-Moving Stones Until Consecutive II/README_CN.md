# 1040. 移动石子直到连续 II
在一个长度 **无限** 的数轴上，第 `i` 颗石子的位置为 `stones[i]`。如果一颗石子的位置最小/最大，那么该石子被称作 **端点石子** 。

每个回合，你可以将一颗端点石子拿起并移动到一个未占用的位置，使得该石子不再是一颗端点石子。

值得注意的是，如果石子像 `stones = [1,2,5]` 这样，你将 **无法** 移动位于位置 5 的端点石子，因为无论将它移动到任何位置（例如 0 或 3），该石子都仍然会是端点石子。

当你无法进行任何移动时，即，这些石子的位置连续时，游戏结束。

要使游戏结束，你可以执行的最小和最大移动次数分别是多少？ 以长度为 2 的数组形式返回答案：`answer = [minimum_moves, maximum_moves]` 。

#### 示例 1:
<pre>
<strong>输入:</strong> stones = [7,4,9]
<strong>输出:</strong> [1,2]
<strong>解释:</strong> 我们可以移动一次，4 -> 8，游戏结束。
或者，我们可以移动两次 9 -> 5，4 -> 6，游戏结束。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> stones = [6,5,4,3,10]
<strong>输出:</strong> [2,3]
<strong>解释:</strong>
我们可以移动 3 -> 8，接着是 10 -> 7，游戏结束。
或者，我们可以移动 3 -> 7, 4 -> 8, 5 -> 9，游戏结束。
注意，我们无法进行 10 -> 2 这样的移动来结束游戏，因为这是不合要求的移动。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> [100,101,104,102,103]
<strong>输出:</strong> [0,0]
</pre>

#### 提示:
* <code>3 <= stones.length <= 10<sup>4</sup></code>
* <code>1 <= stones[i] <= 10<sup>9</sup></code>
* `stones[i]` 的值各不相同。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn num_moves_stones_ii(stones: Vec<i32>) -> Vec<i32> {
        let n = stones.len();
        let mut stones = stones;
        let mut i = 0;
        let mut answer = vec![i32::MAX, 0];

        stones.sort_unstable();

        for j in 0..n {
            while i < n && stones[i] <= stones[j] + n as i32 - 1 {
                i += 1;
            }
            answer[0] = answer[0].min((j + n - i) as i32);
        }
        if answer[0] == 1
            && ((stones[n - 2] - stones[0] + 2 == n as i32 && stones[n - 1] - stones[n - 2] > 2)
                || (stones[n - 1] - stones[1] + 2 == n as i32 && stones[1] - stones[0] > 2))
        {
            answer[0] = 2;
        }

        answer[1] = stones[n - 1] - stones[0] - n as i32 + 2
            - (stones[1] - stones[0]).min(stones[n - 1] - stones[n - 2]);

        answer
    }
}
```
