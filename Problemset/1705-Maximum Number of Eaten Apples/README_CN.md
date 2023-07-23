# 1705. 吃苹果的最大数目
有一棵特殊的苹果树，一连 `n` 天，每天都可以长出若干个苹果。在第 `i` 天，树上会长出 `apples[i]` 个苹果，这些苹果将会在 `days[i]` 天后（也就是说，第 `i + days[i]` 天时）腐烂，变得无法食用。也可能有那么几天，树上不会长出新的苹果，此时用 `apples[i] == 0` 且 `days[i] == 0` 表示。

你打算每天 **最多** 吃一个苹果来保证营养均衡。注意，你可以在这 `n` 天之后继续吃苹果。

给你两个长度为 `n` 的整数数组 `days` 和 `apples` ，返回你可以吃掉的苹果的最大数目。

#### 示例 1:
<pre>
<strong>输入:</strong> apples = [1,2,3,5,2], days = [3,2,1,4,2]
<strong>输出:</strong> 7
<strong>解释:</strong> 你可以吃掉 7 个苹果：
- 第一天，你吃掉第一天长出来的苹果。
- 第二天，你吃掉一个第二天长出来的苹果。
- 第三天，你吃掉一个第二天长出来的苹果。过了这一天，第三天长出来的苹果就已经腐烂了。
- 第四天到第七天，你吃的都是第四天长出来的苹果。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> apples = [3,0,0,0,0,2], days = [3,0,0,0,0,2]
<strong>输出:</strong> 5
<strong>解释:</strong> 你可以吃掉 5 个苹果：
- 第一天到第三天，你吃的都是第一天长出来的苹果。
- 第四天和第五天不吃苹果。
- 第六天和第七天，你吃的都是第六天长出来的苹果。
</pre>

#### 提示:
* `apples.length == n`
* `days.length == n`
* <code>1 <= n <= 2 * 10<sup>4</sup></code>
* <code>0 <= apples[i], days[i] <= 2 * 10<sup>4</sup></code>
* 只有在 `apples[i] = 0` 时，`days[i] = 0` 才成立

## 题解 (Rust)

### 1. 堆
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        let mut i = 0;
        let mut max_day = days.len() as i32;
        let mut ret = 0;

        while i <= max_day {
            if (i as usize) < days.len() {
                heap.push((-i - days[i as usize], apples[i as usize]));
                max_day = max_day.max(i + days[i as usize]);
            }

            while let Some((day, apple)) = heap.pop() {
                if -day > i && apple > 0 {
                    heap.push((day, apple - 1));
                    ret += 1;
                    break;
                }
            }

            i += 1;
        }

        ret
    }
}
```
