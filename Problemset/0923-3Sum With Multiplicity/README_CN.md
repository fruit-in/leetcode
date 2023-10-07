# 923. 三数之和的多种可能
给定一个整数数组 `arr` ，以及一个整数 `target` 作为目标值，返回满足 `i < j < k` 且 `arr[i] + arr[j] + arr[k] == target` 的元组 `i, j, k` 的数量。

由于结果会非常大，请返回 <code>10<sup>9</sup> + 7</code> 的模。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [1,1,2,2,3,3,4,4,5,5], target = 8
<strong>输出:</strong> 20
<strong>解释:</strong>
按值枚举(arr[i], arr[j], arr[k])：
(1, 2, 5) 出现 8 次；
(1, 3, 4) 出现 8 次；
(2, 2, 4) 出现 2 次；
(2, 3, 3) 出现 2 次。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,1,2,2,2,2], target = 5
<strong>输出:</strong> 12
<strong>解释:</strong>
arr[i] = 1, arr[j] = arr[k] = 2 出现 12 次：
我们从 [1,1] 中选择一个 1，有 2 种情况，
从 [2,2,2,2] 中选出两个 2，有 6 种情况。
</pre>

#### 提示:
* `3 <= arr.length <= 3000`
* `0 <= arr[i] <= 100`
* `0 <= target <= 300`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        let mut hashmap = HashMap::new();
        let mut ret = 0;

        for j in 0..arr.len() {
            ret = (ret + hashmap.get(&(target - arr[j])).unwrap_or(&0)) % 1_000_000_007;

            for i in 0..j {
                hashmap
                    .entry(arr[i] + arr[j])
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }

        ret
    }
}
```
