# 137. 只出现一次的数字 II
给你一个整数数组 `nums` ，除某个元素仅出现 **一次** 外，其余每个元素都恰出现 **三次** 。请你找出并返回那个只出现了一次的元素。

你必须设计并实现线性时间复杂度的算法且使用常数级空间来解决此问题。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,2,3,2]
<strong>输出:</strong> 3
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [0,1,0,1,0,1,99]
<strong>输出:</strong> 99
</pre>

#### 提示:
* <code>1 <= nums.length <= 3 * 10<sup>4</sup></code>
* <code>-2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1</code>
* `nums` 中，除某个元素仅出现 **一次** 外，其余每个元素都恰出现 **三次**

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut bitcount = [0; 32];
        let mut ret = 0;

        for num in &nums {
            for i in 0..32 {
                if (num >> i) & 1 == 1 {
                    bitcount[i] += 1;
                }
            }
        }

        for i in 0..32 {
            if bitcount[i] % 3 == 1 {
                ret |= 1 << i;
            }
        }

        ret
    }
}
```
