# 41. 缺失的第一个正数
给你一个未排序的整数数组 `nums` ，请你找出其中没有出现的最小的正整数。

请你实现时间复杂度为 `O(n)` 并且只使用常数级别额外空间的解决方案。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,0]
<strong>输出:</strong> 3
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [3,4,-1,1]
<strong>输出:</strong> 2
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [7,8,9,11,12]
<strong>输出:</strong> 1
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>-2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1</code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        nums.push(0);

        for i in 0..nums.len() {
            while nums[i] >= 0 && nums[i] < nums.len() as i32 && nums[nums[i] as usize] != nums[i] {
                let tmp = nums[i] as usize;
                nums.swap(tmp, i);
            }
        }

        for i in 1..nums.len() {
            if i as i32 != nums[i] {
                return i as i32;
            }
        }

        nums.len() as i32
    }
}
```
