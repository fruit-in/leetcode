# 713. 乘积小于K的子数组
给定一个正整数数组 `nums`。

找出该数组内乘积小于 `k` 的连续的子数组的个数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [10, 5, 2, 6], k = 100
<strong>输出:</strong> 8
<strong>解释:</strong> 8个乘积小于100的子数组分别为: [10], [5], [2], [6], [10,5], [5,2], [2,6], [5,2,6]。
需要注意的是 [10,5,2] 并不是乘积小于100的子数组。
</pre>

#### 说明:
* `0 < nums.length <= 50000`
* `0 < nums[i] < 1000`
* `0 <= k < 10^6`

## 题解 (Ruby)

### 1. 双指针
```Ruby
# @param {Integer[]} nums
# @param {Integer} k
# @return {Integer}
def num_subarray_product_less_than_k(nums, k)
  return 0 if k < 2

  product = 1
  i = 0
  ret = 0

  (0...nums.size).each do |j|
    product *= nums[j]
    while product >= k
      product /= nums[i]
      i += 1
    end
    ret += j - i + 1
  end

  ret
end
```

## 题解 (Rust)

### 1. 双指针
```Rust
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k < 2 {
            return 0;
        }

        let mut product = 1;
        let mut i = 0;
        let mut ret = 0;

        for j in 0..nums.len() {
            product *= nums[j];
            while product >= k {
                product /= nums[i];
                i += 1;
            }
            ret += j - i + 1;
        }

        ret as i32
    }
}
```
