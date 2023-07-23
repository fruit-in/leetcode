# 1470. 重新排列数组
给你一个数组 `nums` ，数组中有 `2n` 个元素，按 <code>[x<sub>1</sub>,x<sub>2</sub>,...,x<sub>n</sub>,y<sub>1</sub>,y<sub>2</sub>,...,y<sub>n</sub>]</code> 的格式排列。

请你将数组按 <code>[x<sub>1</sub>,y<sub>1</sub>,x<sub>2</sub>,y<sub>2</sub>,...,x<sub>n</sub>,y<sub>n</sub>]</code> 格式重新排列，返回重排后的数组。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,5,1,3,4,7], n = 3
<strong>输出:</strong> [2,3,5,4,1,7]
<strong>解释:</strong> 由于 x<sub>1</sub>=2, x<sub>2</sub>=5, x<sub>3</sub>=1, y<sub>1</sub>=3, y<sub>2</sub>=4, y<sub>3</sub>=7 ，所以答案为 [2,3,5,4,1,7]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,4,4,3,2,1], n = 4
<strong>输出:</strong> [1,4,2,3,3,2,4,1]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,1,2,2], n = 2
<strong>输出:</strong> [1,2,1,2]
</pre>

#### 提示:
* `1 <= n <= 500`
* `nums.length == 2n`
* `1 <= nums[i] <= 10^3`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut ret = Vec::with_capacity(2 * n as usize);

        for i in 0..(n as usize) {
            ret.push(nums[i]);
            ret.push(nums[n as usize + i]);
        }

        ret
    }
}
```
