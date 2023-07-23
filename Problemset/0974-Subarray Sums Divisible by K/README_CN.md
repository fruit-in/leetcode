# 974. 和可被 K 整除的子数组
给定一个整数数组 `A`，返回其中元素之和可被 `K` 整除的（连续、非空）子数组的数目。

#### 示例:
<pre>
<strong>Input:</strong> A = [4,5,0,-2,-3,1], K = 5
<strong>Output:</strong> 7
<strong>Explanation:</strong>
有 7 个子数组满足其元素之和可被 K = 5 整除：
[4, 5, 0, -2, -3, 1], [5], [5, 0], [5, 0, -2, -3], [0], [0, -2, -3], [-2, -3]
</pre>

#### 提示:
1. `1 <= A.length <= 30000`
2. `-10000 <= A[i] <= 10000`
3. `2 <= K <= 10000`

## 题解 (Ruby)

### 1. 前缀和
```Ruby
# @param {Integer[]} a
# @param {Integer} k
# @return {Integer}
def subarrays_div_by_k(a, k)
  counter = { 0 => 1 }
  counter.default = 0
  sum = 0
  ret = 0

  a.each do |x|
    sum = (sum + x) % k
    ret += counter[sum]
    counter[sum] += 1
  end

  ret
end
```

## 题解 (Rust)

### 1. 前缀和
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut counter = vec![(0, 1)].into_iter().collect::<HashMap<_, _>>();
        let mut sum = 0;
        let mut ret = 0;

        for x in nums {
            sum = ((sum + x) % k + k) % k;
            let count = counter.entry(sum).or_insert(0);
            ret += *count;
            *count += 1;
        }

        ret
    }
}
```
