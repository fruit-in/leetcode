# 1822. Sign of the Product of an Array
There is a function `signFunc(x)` that returns:
* `1` if `x` is positive.
* `-1` if `x` is negative.
* `0` if `x` is equal to `0`.

You are given an integer array `nums`. Let `product` be the product of all values in the array `nums`.

Return `signFunc(product)`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [-1,-2,-3,-4,3,2,1]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The product of all values in the array is 144, and signFunc(144) = 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,5,0,2,-3]
<strong>Output:</strong> 0
<strong>Explanation:</strong> The product of all values in the array is 0, and signFunc(0) = 0
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [-1,1,-1,1,-1]
<strong>Output:</strong> -1
<strong>Explanation:</strong> The product of all values in the array is -1, and signFunc(-1) = -1
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* `-100 <= nums[i] <= 100`

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def array_sign(nums)
  sign = 1

  nums.each do |x|
    return 0 if x == 0

    sign *= -1 if x < 0
  end

  sign
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut sign = 1;

        for x in nums {
            if x == 0 {
                return 0;
            } else if x < 0 {
                sign *= -1;
            }
        }

        sign
    }
}
```
