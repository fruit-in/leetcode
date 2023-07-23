# 1512. Number of Good Pairs
Given an array of integers `nums`.

A pair `(i,j)` is called *good* if `nums[i]` == `nums[j]` and `i` < `j`.

Return the number of *good* pairs.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3,1,1,3]
<strong>Output:</strong> 4
<strong>Explanation:</strong> There are 4 good pairs (0,3), (0,4), (3,4), (2,5) 0-indexed.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,1,1,1]
<strong>Output:</strong> 6
<strong>Explanation:</strong> Each pair in the array are <em>good</em>.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,2,3]
<strong>Output:</strong> 0
</pre>

#### Constraints:
* `1 <= nums.length <= 100`
* `1 <= nums[i] <= 100`

## Solutions (Ruby)

### 1. Count
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def num_identical_pairs(nums)
    cnt = [0] * 101
    ret = 0

    for num in nums
        ret += cnt[num]
        cnt[num] += 1
    end

    return ret
end
```

## Solutions (Rust)

### 1. Count
```Rust
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut cnt = [0; 101];
        let mut ret = 0;

        for num in nums {
            ret += cnt[num as usize];
            cnt[num as usize] += 1;
        }

        ret
    }
}
```
