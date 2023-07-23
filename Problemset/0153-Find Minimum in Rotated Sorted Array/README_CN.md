# 153. 寻找旋转排序数组中的最小值
已知一个长度为 `n` 的数组，预先按照升序排列，经由 `1` 到 `n` 次 **旋转** 后，得到输入数组。例如，原数组 `nums = [0,1,2,4,5,6,7]` 在变化后可能得到：
* 若旋转 `4` 次，则可以得到 `[4,5,6,7,0,1,2]`
* 若旋转 `7` 次，则可以得到 `[0,1,2,4,5,6,7]`

注意，数组 `[a[0], a[1], a[2], ..., a[n-1]]` **旋转一次** 的结果为数组 `[a[n-1], a[0], a[1], a[2], ..., a[n-2]]` 。

给你一个元素值 **互不相同** 的数组 `nums` ，它原来是一个升序排列的数组，并按上述情形进行了多次旋转。请你找出并返回数组中的 **最小元素** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,4,5,1,2]
<strong>输出:</strong> 1
<strong>解释:</strong> 原数组为 [1,2,3,4,5] ，旋转 3 次得到输入数组。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [4,5,6,7,0,1,2]
<strong>输出:</strong> 0
<strong>解释:</strong> 原数组为 [0,1,2,4,5,6,7] ，旋转 4 次得到输入数组。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [11,13,15,17]
<strong>输出:</strong> 11
<strong>解释:</strong> 原数组为 [11,13,15,17] ，旋转 4 次得到输入数组。
</pre>

#### 提示:
* `n == nums.length`
* `1 <= n <= 5000`
* `-5000 <= nums[i] <= 5000`
* `nums` 中的所有整数 **互不相同**
* `nums` 原来是一个升序排序的数组，并进行了 `1` 至 `n` 次旋转

## 题解 (Ruby)

### 1. 二分查找
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def find_min(nums)
  l = 0
  r = nums.size - 1

  while l <= r
    m = (l + r) / 2

    if m > 0 && nums[m - 1] > nums[m]
      return nums[m]
    elsif nums[m] >= nums[0]
      l = m + 1
    else
      r = m - 1
    end
  end

  nums[0]
end
```

## 题解 (Rust)

### 1. 二分查找
```Rust
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;

        while l <= r {
            let m = (l + r) / 2;

            if m > 0 && nums[m - 1] > nums[m] {
                return nums[m];
            } else if nums[m] >= nums[0] {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }

        nums[0]
    }
}
```
