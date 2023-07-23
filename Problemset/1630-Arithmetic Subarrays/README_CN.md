# 1630. 等差子数组
如果一个数列由至少两个元素组成，且每两个连续元素之间的差值都相同，那么这个序列就是 **等差数列** 。更正式地，数列 `s` 是等差数列，只需要满足：对于每个有效的 `i` ， `s[i+1] - s[i] == s[1] - s[0]` 都成立。

例如，下面这些都是 **等差数列** ：
```
1, 3, 5, 7, 9
7, 7, 7, 7
3, -1, -5, -9
```

下面的数列 **不是等差数列** ：
```
1, 1, 2, 5, 7
```

给你一个由 `n` 个整数组成的数组 `nums`，和两个由 `m` 个整数组成的数组 `l` 和 `r`，后两个数组表示 `m` 组范围查询，其中第 `i` 个查询对应范围 `[l[i], r[i]]` 。所有数组的下标都是 **从 0 开始** 的。

返回 `boolean` 元素构成的答案列表 `answer` 。如果子数组 `nums[l[i]], nums[l[i]+1], ... , nums[r[i]]` 可以 **重新排列** 形成 **等差数列** ，`answer[i]` 的值就是 `true`；否则`answer[i]` 的值就是 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [4,6,5,9,3,7], l = [0,0,2], r = [2,3,5]
<strong>输出:</strong> [true,false,true]
<strong>解释:</strong>
第 0 个查询，对应子数组 [4,6,5] 。可以重新排列为等差数列 [6,5,4] 。
第 1 个查询，对应子数组 [4,6,5,9] 。无法重新排列形成等差数列。
第 2 个查询，对应子数组 [5,9,3,7] 。可以重新排列为等差数列 [3,5,7,9] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [-12,-9,-3,-12,-6,15,20,-25,-20,-15,-10], l = [0,1,6,4,8,7], r = [4,4,9,7,9,10]
<strong>输出:</strong> [false,true,false,false,true,true]
</pre>

#### 提示:
* `n == nums.length`
* `m == l.length`
* `m == r.length`
* `2 <= n <= 500`
* `1 <= m <= 500`
* `0 <= l[i] < r[i] < n`
* <code>-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Ruby)

### 1. 排序
```Ruby
# @param {Integer[]} nums
# @param {Integer[]} l
# @param {Integer[]} r
# @return {Boolean[]}
def check_arithmetic_subarrays(nums, l, r)
  ret = [false] * l.size

  (0...l.size).each do |i|
    sub = nums[l[i]..r[i]].sort
    ret[i] = sub.size > 1 && (2...sub.size).all? { |j| sub[j] - sub[j - 1] == sub[1] - sub[0] }
  end

  ret
end
```

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut ret = vec![false; l.len()];

        for i in 0..l.len() {
            let mut sub = nums[l[i] as usize..=r[i] as usize].to_vec();
            sub.sort_unstable();
            ret[i] =
                sub.len() > 1 && (2..sub.len()).all(|j| sub[j] - sub[j - 1] == sub[1] - sub[0]);
        }

        ret
    }
}
```
