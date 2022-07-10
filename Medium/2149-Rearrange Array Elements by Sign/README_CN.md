# 2149. 按符号重排数组
给你一个下标从 **0** 开始的整数数组 `nums` ，数组长度为 **偶数** ，由数目相等的正整数和负整数组成。

你需要 **重排** `nums` 中的元素，使修改后的数组满足下述条件：
* 任意 **连续** 的两个整数 **符号相反**
* 对于符号相同的所有整数，**保留** 它们在 `nums` 中的 顺序 。
* 重排后数组以正整数开头。

重排元素满足上述条件后，返回修改后的数组。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,1,-2,-5,2,-4]
<strong>输出:</strong> [3,-2,1,-5,2,-4]
<strong>解释:</strong>
nums 中的正整数是 [3,1,2] ，负整数是 [-2,-5,-4] 。
重排的唯一可行方案是 [3,-2,1,-5,2,-4]，能满足所有条件。
像 [1,-2,2,-5,3,-4]、[3,1,2,-2,-5,-4]、[-2,3,-5,1,-4,2] 这样的其他方案是不正确的，因为不满足一个或者多个条件。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [-1,1]
<strong>输出:</strong> [1,-1]
<strong>解释:</strong>
1 是 nums 中唯一一个正整数，-1 是 nums 中唯一一个负整数。
所以 nums 重排为 [1,-1] 。
</pre>

#### 提示:
* <code>2 <= nums.length <= 2 * 10<sup>5</sup></code>
* `nums.length` 是 **偶数**
* <code>1 <= |nums[i]| <= 10<sup>5</sup></code>
* `nums` 由 **相等** 数量的正整数和负整数组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ps = vec![];
        let mut ns = vec![];
        let mut ret = vec![];

        for num in nums {
            if num > 0 {
                ps.push(num);
            } else {
                ns.push(num);
            }
        }

        for i in 0..ps.len() {
            ret.push(ps[i]);
            ret.push(ns[i]);
        }

        ret
    }
}
```
