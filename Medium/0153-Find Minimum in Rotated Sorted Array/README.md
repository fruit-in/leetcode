# 153. Find Minimum in Rotated Sorted Array
Suppose an array of length `n` sorted in ascending order is **rotated** between `1` and `n` times. For example, the array `nums = [0,1,2,4,5,6,7]` might become:
* `[4,5,6,7,0,1,2]` if it was rotated `4` times.
* `[0,1,2,4,5,6,7]` if it was rotated `7` times.

Notice that **rotating** an array `[a[0], a[1], a[2], ..., a[n-1]]` 1 time results in the array `[a[n-1], a[0], a[1], a[2], ..., a[n-2]]`.

Given the sorted rotated array `nums` of **unique** elements, return *the minimum element of this array*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,4,5,1,2]
<strong>Output:</strong> 1
<strong>Explanation:</strong> The original array was [1,2,3,4,5] rotated 3 times.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [4,5,6,7,0,1,2]
<strong>Output:</strong> 0
<strong>Explanation:</strong> The original array was [0,1,2,4,5,6,7] and it was rotated 4 times.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [11,13,15,17]
<strong>Output:</strong> 11
<strong>Explanation:</strong> The original array was [11,13,15,17] and it was rotated 4 times.
</pre>

#### Constraints:
* `n == nums.length`
* `1 <= n <= 5000`
* `-5000 <= nums[i] <= 5000`
* All the integers of `nums` are **unique**.
* `nums` is sorted and rotated between `1` and `n` times.

## Solutions (Ruby)

### 1. Binary Search
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

## Solutions (Rust)

### 1. Binary Search
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
