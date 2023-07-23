# 1658. 将 x 减到 0 的最小操作数
给你一个整数数组 `nums` 和一个整数 `x` 。每一次操作时，你应当移除数组 `nums` 最左边或最右边的元素，然后从 `x` 中减去该元素的值。请注意，需要 **修改** 数组以供接下来的操作使用。

如果可以将 `x` **恰好** 减到 `0` ，返回 **最小操作数** ；否则，返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,1,4,2,3], x = 5
<strong>输出:</strong> 2
<strong>解释:</strong> 最佳解决方案是移除后两个元素，将 x 减到 0 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [5,6,7,8,9], x = 4
<strong>输出:</strong> -1
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [3,2,20,1,1,3], x = 10
<strong>输出:</strong> 5
<strong>解释:</strong> 最佳解决方案是移除后三个元素和前两个元素（总共 5 次操作），将 x 减到 0 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>4</sup></code>
* <code>1 <= x <= 10<sup>9</sup></code>

## 题解 (Ruby)

### 1. 双指针
```Ruby
# @param {Integer[]} nums
# @param {Integer} x
# @return {Integer}
def min_operations(nums, x)
  l = nums.size
  r = 0
  sum = nums.sum
  ret = -1

  while r < nums.size
    ret = l + r if sum == x && (ret == -1 || l + r < ret)
    if (sum > x && l > 0) || l + r >= nums.size
      l -= 1
      sum -= nums[l]
    else
      r += 1
      sum += nums[-r]
    end
  end

  ret
end
```

## 题解 (Rust)

### 1. 双指针
```Rust
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut l = nums.len();
        let mut r = 0;
        let mut sum = nums.iter().sum::<i32>();
        let mut ret = -1;

        while r < nums.len() {
            if sum == x && (ret == -1 || l + r < ret as usize) {
                ret = (l + r) as i32;
            }
            if (sum > x && l > 0) || l + r >= nums.len() {
                l -= 1;
                sum -= nums[l];
            } else {
                r += 1;
                sum += nums[nums.len() - r];
            }
        }

        ret
    }
}
```
