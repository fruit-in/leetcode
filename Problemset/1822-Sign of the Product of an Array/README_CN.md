# 1822. 数组元素积的符号
已知函数 `signFunc(x)` 将会根据 `x` 的正负返回特定值：
* 如果 `x` 是正数，返回 `1` 。
* 如果 `x` 是负数，返回 `-1` 。
* 如果 `x` 是等于 `0` ，返回 `0` 。

给你一个整数数组 `nums` 。令 `product` 为数组 `nums` 中所有元素值的乘积。

返回 `signFunc(product)` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [-1,-2,-3,-4,3,2,1]
<strong>输出:</strong> 1
<strong>解释:</strong> 数组中所有值的乘积是 144 ，且 signFunc(144) = 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,5,0,2,-3]
<strong>输出:</strong> 0
<strong>解释:</strong> 数组中所有值的乘积是 0 ，且 signFunc(0) = 0
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [-1,1,-1,1,-1]
<strong>输出:</strong> -1
<strong>解释:</strong> 数组中所有值的乘积是 -1 ，且 signFunc(-1) = -1
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* `-100 <= nums[i] <= 100`

## 题解 (Ruby)

### 1. 题解
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

## 题解 (Rust)

### 1. 题解
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
