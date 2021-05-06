# 713. Subarray Product Less Than K
Your are given an array of positive integers `nums`.

Count and print the number of (contiguous) subarrays where the product of all the elements in the subarray is less than `k`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [10, 5, 2, 6], k = 100
<strong>Output:</strong> 8
<strong>Explanation:</strong> The 8 subarrays that have product less than 100 are: [10], [5], [2], [6], [10, 5], [5, 2], [2, 6], [5, 2, 6].
Note that [10, 5, 2] is not included as the product of 100 is not strictly less than k.
</pre>

#### Note:
* `0 < nums.length <= 50000`.
* `0 < nums[i] < 1000`.
* `0 <= k < 10^6`.

## Solutions (Ruby)

### 1. Two Pointers
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

## Solutions (Rust)

### 1. Two Pointers
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
