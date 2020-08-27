# 1512. 好数对的数目
给你一个整数数组 `nums` 。

如果一组数字 `(i,j)` 满足 `nums[i]` == `nums[j]` 且 `i` < `j` ，就可以认为这是一组 **好数对** 。

返回好数对的数目。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,1,1,3]
<strong>输出:</strong> 4
<strong>解释:</strong> 有 4 组好数对，分别是 (0,3), (0,4), (3,4), (2,5) ，下标从 0 开始
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,1,1,1]
<strong>输出:</strong> 6
<strong>解释:</strong> 数组中的每组数字都是好数对
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,2,3]
<strong>输出:</strong> 0
</pre>

#### 提示:
* `1 <= nums.length <= 100`
* `1 <= nums[i] <= 100`

## 题解 (Ruby)

### 1. 计数
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

## 题解 (Rust)

### 1. 计数
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
