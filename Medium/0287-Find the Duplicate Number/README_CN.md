# 287. 寻找重复数
给定一个包含 *n* + 1 个整数的数组 *nums*，其数字都在 1 到 *n* 之间（包括 1 和 *n*），可知至少存在一个重复的整数。假设只有一个重复的整数，找出这个重复的数。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,3,4,2,2]
<strong>输出:</strong> 2
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [3,1,3,4,2]
<strong>输出:</strong> 3
</pre>

#### 说明:
1. **不能**更改原数组（假设数组是只读的）。
2. 只能使用额外的 *O*(1) 的空间。
3. 时间复杂度小于 *O*(*n*<sup>2</sup>) 。
4. 数组中只有一个重复的数字，但它可能不止重复出现一次。

## 题解 (Rust)

### 1. 双指针
```Rust
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut p1 = 0;
        let mut p2 = 0;

        loop {
            p1 = nums[p1 as usize];
            p2 = nums[nums[p2 as usize] as usize];
            if p1 == p2 {
                break;
            }
        }

        p1 = 0;
        while p1 != p2 {
            p1 = nums[p1 as usize];
            p2 = nums[p2 as usize];
        }

        p1
    }
}
```
