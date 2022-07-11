# 2239. 找到最接近 0 的数字
给你一个长度为 `n` 的整数数组 `nums` ，请你返回 `nums` 中最 **接近** `0` 的数字。如果有多个答案，请你返回它们中的 **最大值** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [-4,-2,1,4,8]
<strong>输出:</strong> 1
<strong>解释:</strong>
-4 到 0 的距离为 |-4| = 4 。
-2 到 0 的距离为 |-2| = 2 。
1 到 0 的距离为 |1| = 1 。
4 到 0 的距离为 |4| = 4 。
8 到 0 的距离为 |8| = 8 。
所以，数组中距离 0 最近的数字为 1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,-1,1]
<strong>输出:</strong> 1
<strong>解释:</strong> 1 和 -1 都是距离 0 最近的数字，所以返回较大值 1 。
</pre>

#### 提示:
* `1 <= n <= 1000`
* <code>-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .min_by_key(|num| (num.abs(), -num))
            .unwrap()
    }
}
```
