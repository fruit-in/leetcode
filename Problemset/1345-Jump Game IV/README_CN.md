# 1345. 跳跃游戏 IV
给你一个整数数组 `arr` ，你一开始在数组的第一个元素处（下标为 0）。

每一步，你可以从下标 `i` 跳到下标 `i + 1` 、`i - 1` 或者 `j` ：
* `i + 1` 需满足：`i + 1 < arr.length`
* `i - 1` 需满足：`i - 1 >= 0`
* `j` 需满足：`arr[i] == arr[j]` 且 `i != j`

请你返回到达数组最后一个元素的下标处所需的 **最少操作次数** 。

注意：任何时候你都不能跳到数组外面。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [100,-23,-23,404,100,23,23,23,3,404]
<strong>输出:</strong> 3
<strong>解释:</strong> 那你需要跳跃 3 次，下标依次为 0 --> 4 --> 3 --> 9 。下标 9 为数组的最后一个元素的下标。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [7]
<strong>输出:</strong> 0
<strong>解释:</strong> 一开始就在最后一个元素处，所以你不需要跳跃。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [7,6,9,6,9,6,9,7]
<strong>输出:</strong> 1
<strong>解释:</strong> 你可以直接从下标 0 处跳到下标 7 处，也就是数组的最后一个元素处。
</pre>

#### 提示:
* <code>1 <= arr.length <= 5 * 10<sup>4</sup></code>
* <code>-10<sup>8</sup> <= arr[i] <= 10<sup>8</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut js = HashMap::new();
        let mut visited = HashSet::from([0]);
        let mut deque = VecDeque::from([(0, 0)]);

        for i in 0..arr.len() {
            js.entry(arr[i]).or_insert(vec![]).push(i);
        }

        while let Some((i, steps)) = deque.pop_front() {
            if i == arr.len() - 1 {
                return steps;
            }

            if i > 0 && !visited.contains(&(i - 1)) {
                visited.insert(i - 1);
                deque.push_back((i - 1, steps + 1));
            }
            if i < arr.len() - 1 && !visited.contains(&(i + 1)) {
                visited.insert(i + 1);
                deque.push_back((i + 1, steps + 1));
            }
            for j in js.remove(&arr[i]).unwrap_or(vec![]) {
                if !visited.contains(&j) {
                    visited.insert(j);
                    deque.push_back((j, steps + 1));
                }
            }
        }

        unreachable!()
    }
}
```
