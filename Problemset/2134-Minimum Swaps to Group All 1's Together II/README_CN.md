# 2134. 最少交换次数来组合所有的 1 II
**交换** 定义为选中一个数组中的两个 **互不相同** 的位置并交换二者的值。

**环形** 数组是一个数组，可以认为 **第一个** 元素和 **最后一个** 元素 **相邻** 。

给你一个 **二进制环形** 数组 `nums` ，返回在 **任意位置** 将数组中的所有 `1` 聚集在一起需要的最少交换次数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [0,1,0,1,1,0,0]
<strong>输出:</strong> 1
<strong>解释:</strong> 这里列出一些能够将所有 1 聚集在一起的方案：
[0,0,1,1,1,0,0] 交换 1 次。
[0,1,1,1,0,0,0] 交换 1 次。
[1,1,0,0,0,0,1] 交换 2 次（利用数组的环形特性）。
无法在交换 0 次的情况下将数组中的所有 1 聚集在一起。
因此，需要的最少交换次数为 1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [0,1,1,1,0,0,1,1,0]
<strong>输出:</strong> 2
<strong>解释:</strong> 这里列出一些能够将所有 1 聚集在一起的方案：
[1,1,1,0,0,0,0,1,1] 交换 2 次（利用数组的环形特性）。
[1,1,1,1,1,0,0,0,0] 交换 2 次。
无法在交换 0 次或 1 次的情况下将数组中的所有 1 聚集在一起。
因此，需要的最少交换次数为 2 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,1,0,0,1]
<strong>输出:</strong> 0
<strong>解释:</strong> 得益于数组的环形特性，所有的 1 已经聚集在一起。
因此，需要的最少交换次数为 0 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* `nums[i]` 为 `0` 或者 `1`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let count_1 = nums.iter().filter(|&&x| x == 1).count();
        let mut count_0 = nums.iter().take(count_1).filter(|&&x| x == 0).count();
        let mut ret = count_0;

        for i in 0..nums.len() - 1 {
            count_0 -= (nums[i] == 0) as usize;
            count_0 += (nums[(i + count_1) % nums.len()] == 0) as usize;
            ret = ret.min(count_0);
        }

        ret as i32
    }
}
```
