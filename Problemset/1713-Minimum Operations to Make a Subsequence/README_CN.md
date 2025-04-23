# 1713. 得到子序列的最少操作次数
给你一个数组 `target` ，包含若干 **互不相同** 的整数，以及另一个整数数组 `arr` ，`arr` **可能** 包含重复元素。

每一次操作中，你可以在 `arr` 的任意位置插入任一整数。比方说，如果 `arr = [1,4,1,2]` ，那么你可以在中间添加 `3` 得到 `[1,4,3,1,2]` 。你可以在数组最开始或最后面添加整数。

请你返回 **最少** 操作次数，使得 `target` 成为 `arr` 的一个子序列。

一个数组的 **子序列** 指的是删除原数组的某些元素（可能一个元素都不删除），同时不改变其余元素的相对顺序得到的数组。比方说，`[2,7,4]` 是 `[4,2,3,7,2,1,4]` 的子序列（加粗元素），但 `[2,4,2]` 不是子序列。

#### 示例 1:
<pre>
<strong>输入:</strong> target = [5,1,3], arr = [9,4,2,3,4]
<strong>输出:</strong> 2
<strong>解释:</strong> 你可以添加 5 和 1 ，使得 arr 变为 [5,9,4,1,2,3,4] ，target 为 arr 的子序列。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> target = [6,4,8,1,3,2], arr = [4,7,6,2,3,8,6,1]
<strong>输出:</strong> 3
</pre>

#### 提示:
* <code>1 <= target.length, arr.length <= 10<sup>5</sup></code>
* <code>1 <= target[i], arr[i] <= 10<sup>9</sup></code>
* `target` 不包含任何重复元素。

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
        let target = target
            .iter()
            .enumerate()
            .map(|(i, x)| (x, i))
            .collect::<HashMap<_, _>>();
        let arr = arr
            .iter()
            .filter(|x| target.contains_key(x))
            .map(|x| target[&x])
            .collect::<Vec<_>>();
        let mut lis = vec![];

        for i in &arr {
            if let Err(j) = lis.binary_search(&i) {
                if j == lis.len() {
                    lis.push(i);
                } else {
                    lis[j] = i;
                }
            }
        }

        (target.len() - lis.len()) as i32
    }
}
```
