# 1191. K 次串联后最大子数组之和
给你一个整数数组 `arr` 和一个整数 `k`。

首先，我们要对该数组进行修改，即把原数组 `arr` 重复 `k` 次。

> 举个例子，如果 `arr = [1, 2]` 且 `k = 3`，那么修改后的数组就是 `[1, 2, 1, 2, 1, 2]`。

然后，请你返回修改后的数组中的最大的子数组之和。

注意，子数组长度可以是 `0`，在这种情况下它的总和也是 `0`。

由于 **结果可能会很大**，所以需要 **模（mod）** `10^9 + 7` 后再返回。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [1,2], k = 3
<strong>输出:</strong> 9
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,-2,1], k = 5
<strong>输出:</strong> 2
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> arr = [-1,-2], k = 7
<strong>输出:</strong> 0
</pre>

#### 提示:
* `1 <= arr.length <= 10^5`
* `1 <= k <= 10^5`
* `-10^4 <= arr[i] <= 10^4`

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[]} arr
# @param {Integer} k
# @return {Integer}
def k_concatenation_max_sum(arr, k)
  l_sum = 0
  r_sum = 0
  l_max_sum = 0
  r_max_sum = 0
  l_min_sum = 0
  ret = 0

  (1..arr.size).each do |i|
    l_sum += arr[i - 1]
    r_sum += arr[-i]
    l_max_sum = [l_max_sum, l_sum].max
    r_max_sum = [r_max_sum, r_sum].max
    l_min_sum = [l_min_sum, l_sum].min
    ret = [ret, l_sum - l_min_sum].max
  end

  k == 1 ? ret : [ret, [l_sum, 0].max * (k - 2) + l_max_sum + r_max_sum].max % 1_000_000_007
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        let mut l_sum = 0i64;
        let mut r_sum = 0i64;
        let mut l_max_sum = 0i64;
        let mut r_max_sum = 0i64;
        let mut l_min_sum = 0i64;
        let mut ret = 0i64;

        for i in 0..arr.len() {
            l_sum += arr[i] as i64;
            r_sum += arr[arr.len() - 1 - i] as i64;
            l_max_sum = l_max_sum.max(l_sum);
            r_max_sum = r_max_sum.max(r_sum);
            l_min_sum = l_min_sum.min(l_sum);
            ret = ret.max(l_sum - l_min_sum);
        }

        match k {
            1 => ret as i32,
            _ => {
                (ret.max(l_sum.max(0) * (k as i64 - 2) + l_max_sum + r_max_sum) % 1_000_000_007)
                    as i32
            }
        }
    }
}
```
