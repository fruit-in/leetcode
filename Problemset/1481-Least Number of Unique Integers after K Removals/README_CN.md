# 1481. 不同整数的最少数目
给你一个整数数组 `arr` 和一个整数 `k` 。现需要从数组中恰好移除 `k` 个元素，请找出移除后数组中不同整数的最少数目。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [5,5,4], k = 1
<strong>输出:</strong> 1
<strong>解释:</strong> 移除 1 个 4 ，数组中只剩下 5 一种整数。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [4,3,1,1,3,3,2], k = 3
<strong>输出:</strong> 2
<strong>解释:</strong> 先移除 4、2 ，然后再移除两个 1 中的任意 1 个或者三个 3 中的任意 1 个，最后剩下 1 和 3 两种整数。
</pre>

#### 提示:
* `1 <= arr.length <= 10^5`
* `1 <= arr[i] <= 10^9`
* `0 <= k <= arr.length`

## 题解 (Ruby)

### 1. 排序
```Ruby
# @param {Integer[]} arr
# @param {Integer} k
# @return {Integer}
def find_least_num_of_unique_ints(arr, k)
  counter = {}
  counter.default = 0
  arr.each { |x| counter[x] += 1 }
  counter = counter.values.sort

  (0..counter.size).each do |i|
    if k == 0
      return counter.size - i
    elsif k < 0
      return counter.size - i + 1
    end

    k -= counter[i]
  end
end
```

## 题解 (Rust)

### 1. 排序
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut counter = HashMap::new();
        for x in arr {
            *counter.entry(x).or_insert(0) += 1;
        }

        let mut counter = counter.values().collect::<Vec<_>>();
        counter.sort_unstable();

        for i in 0..=counter.len() {
            if k == 0 {
                return (counter.len() - i) as i32;
            } else if k < 0 {
                return (counter.len() - i) as i32 + 1;
            }
            k -= counter[i];
        }

        0
    }
}
```
