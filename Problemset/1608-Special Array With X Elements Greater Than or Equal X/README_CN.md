# 1608. 特殊数组的特征值
给你一个非负整数数组 `nums` 。如果存在一个数 `x` ，使得 `nums` 中恰好有 `x` 个元素 **大于或者等于** `x` ，那么就称 `nums` 是一个 **特殊数组** ，而 `x` 是该数组的 **特征值** 。

注意： `x` **不必** 是 `nums` 的中的元素。

如果数组 `nums` 是一个 **特殊数组** ，请返回它的特征值 `x` 。否则，返回 `-1` 。可以证明的是，如果 `nums` 是特殊数组，那么其特征值 `x` 是 **唯一的** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,5]
<strong>输出:</strong> 2
<strong>解释:</strong> 有 2 个元素（3 和 5）大于或等于 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [0,0]
<strong>输出:</strong> -1
<strong>解释:</strong> 没有满足题目要求的特殊数组，故而也不存在特征值 x 。
如果 x = 0，应该有 0 个元素 >= x，但实际有 2 个。
如果 x = 1，应该有 1 个元素 >= x，但实际有 0 个。
如果 x = 2，应该有 2 个元素 >= x，但实际有 0 个。
x 不能取更大的值，因为 nums 中只有两个元素。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [0,4,3,0,4]
<strong>输出:</strong> 3
<strong>解释:</strong> 有 3 个元素大于或等于 3 。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> nums = [3,6,7,7,0]
<strong>输出:</strong> -1
</pre>

#### 提示:
* `1 <= nums.length <= 100`
* `0 <= nums[i] <= 1000`

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable_by(|a, b| b.cmp(a));

        for x in 1..=nums.len() {
            if nums[x - 1] >= x as i32 && (x == nums.len() || nums[x] < x as i32) {
                return x as i32;
            }
        }

        -1
    }
}
```
