# 2348. 全 0 子数组的数目
给你一个整数数组 `nums` ，返回全部为 `0` 的 **子数组** 数目。

**子数组** 是一个数组中一段连续非空元素组成的序列。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,3,0,0,2,0,0,4]
<strong>输出:</strong> 6
<strong>解释:</strong>
子数组 [0] 出现了 4 次。
子数组 [0,0] 出现了 2 次。
不存在长度大于 2 的全 0 子数组，所以我们返回 6 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [0,0,0,2,0,0]
<strong>输出:</strong> 9
<strong>解释:</strong>
子数组 [0] 出现了 5 次。
子数组 [0,0] 出现了 3 次。
子数组 [0,0,0] 出现了 1 次。
不存在长度大于 3 的全 0 子数组，所以我们返回 9 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [2,10,2019]
<strong>输出:</strong> 0
<strong>解释:</strong> 没有全 0 子数组，所以我们返回 0 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut count = 0;
        let mut ret = 0;

        for i in 0..nums.len() {
            count += 1;
            if nums[i] != 0 {
                count = 0;
            }
            ret += count;
        }

        ret
    }
}
```
