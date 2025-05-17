# 2607. 使子数组元素和相等
给你一个下标从 **0** 开始的整数数组 `arr` 和一个整数 `k` 。数组 `arr` 是一个循环数组。换句话说，数组中的最后一个元素的下一个元素是数组中的第一个元素，数组中第一个元素的前一个元素是数组中的最后一个元素。

你可以执行下述运算任意次：
* 选中 `arr` 中任意一个元素，并使其值加上 `1` 或减去 `1` 。

执行运算使每个长度为 `k` 的 **子数组** 的元素总和都相等，返回所需要的最少运算次数。

**子数组** 是数组的一个连续部分。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [1,4,1,3], k = 2
<strong>输出:</strong> 1
<strong>解释:</strong> 在下标为 1 的元素那里执行一次运算，使其等于 3 。
执行运算后，数组变为 [1,3,1,3] 。
- 0 处起始的子数组为 [1, 3] ，元素总和为 4
- 1 处起始的子数组为 [3, 1] ，元素总和为 4
- 2 处起始的子数组为 [1, 3] ，元素总和为 4
- 3 处起始的子数组为 [3, 1] ，元素总和为 4
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [2,5,5,7], k = 3
<strong>输出:</strong> 5
<strong>解释:</strong> 在下标为 0 的元素那里执行三次运算，使其等于 5 。在下标为 3 的元素那里执行两次运算，使其等于 5 。
执行运算后，数组变为 [5,5,5,5] 。
- 0 处起始的子数组为 [5, 5, 5] ，元素总和为 15
- 1 处起始的子数组为 [5, 5, 5] ，元素总和为 15
- 2 处起始的子数组为 [5, 5, 5] ，元素总和为 15
- 3 处起始的子数组为 [5, 5, 5] ，元素总和为 15
</pre>

#### 提示:
* <code>1 <= k <= arr.length <= 10<sup>5</sup></code>
* <code>1 <= arr[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn make_sub_k_sum_equal(arr: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let n = arr.len();
        let mut visited = vec![false; n];
        let mut groups = HashMap::new();
        let mut ret = 0;

        for i in 0..n {
            if !visited[i] {
                let mut j = i;
                groups.insert(i, vec![]);

                while !visited[j] {
                    visited[j] = true;
                    groups.get_mut(&i).unwrap().push(arr[j]);
                    j = (j + k) % n;
                }
            }
        }

        for mut group in groups.into_values() {
            let i = group.len() / 2;
            group.sort_unstable();

            for j in 0..group.len() {
                ret += (group[j] - group[i]).abs() as i64;
            }
        }

        ret
    }
}
```
