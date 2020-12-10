# 957. N 天后的牢房
8 间牢房排成一排，每间牢房不是有人住就是空着。

每天，无论牢房是被占用或空置，都会根据以下规则进行更改：
* 如果一间牢房的两个相邻的房间都被占用或都是空的，那么该牢房就会被占用。
* 否则，它就会被空置。

（请注意，由于监狱中的牢房排成一行，所以行中的第一个和最后一个房间无法有两个相邻的房间。）

我们用以下方式描述监狱的当前状态：如果第 `i` 间牢房被占用，则 `cell[i]==1`，否则 `cell[i]==0`。

根据监狱的初始状态，在 `N` 天后返回监狱的状况（和上述 `N` 种变化）。

#### 示例 1:
<pre>
<strong>输入:</strong> cells = [0,1,0,1,1,0,0,1], N = 7
<strong>输出:</strong> [0,0,1,1,0,0,0,0]
<strong>解释:</strong>
下表概述了监狱每天的状况：
Day 0: [0, 1, 0, 1, 1, 0, 0, 1]
Day 1: [0, 1, 1, 0, 0, 0, 0, 0]
Day 2: [0, 0, 0, 0, 1, 1, 1, 0]
Day 3: [0, 1, 1, 0, 0, 1, 0, 0]
Day 4: [0, 0, 0, 0, 0, 1, 0, 0]
Day 5: [0, 1, 1, 1, 0, 1, 0, 0]
Day 6: [0, 0, 1, 0, 1, 1, 0, 0]
Day 7: [0, 0, 1, 1, 0, 0, 0, 0]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> cells = [1,0,0,1,0,0,1,0], N = 1000000000
<strong>输出:</strong> [0,0,1,1,1,1,1,0]
</pre>

#### 提示:
1. `cells.length == 8`
2. `cells[i]` 的值为 `0` 或 `1`
3. `1 <= N <= 10^9`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn prison_after_n_days(mut cells: Vec<i32>, n: i32) -> Vec<i32> {
        Self::prison_after_one_day(&mut cells);

        let origin = cells.clone();
        let mut period = n;

        for i in 1..n {
            Self::prison_after_one_day(&mut cells);

            if cells[1..7] == origin[1..7] {
                period = i;
                break;
            }
        }

        for _ in 0..((n - 1 - period) % period) {
            Self::prison_after_one_day(&mut cells);
        }

        cells
    }

    pub fn prison_after_one_day(cells: &mut Vec<i32>) {
        let next_day = (1..7)
            .map(|i| 1 - (cells[i - 1] ^ cells[i + 1]))
            .collect::<Vec<_>>();

        cells[0] = 0;
        cells[7] = 0;
        for i in 1..7 {
            cells[i] = next_day[i - 1];
        }
    }
}
```
