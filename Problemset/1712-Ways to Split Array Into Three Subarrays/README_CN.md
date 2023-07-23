# 1712. 将数组分成三个子数组的方案数
我们称一个分割整数数组的方案是 **好的** ，当它满足：
* 数组被分成三个 **非空** 连续子数组，从左至右分别命名为 `left` ， `mid` ， `right` 。
* `left` 中元素和小于等于 `mid` 中元素和，`mid` 中元素和小于等于 `right` 中元素和。

给你一个 **非负** 整数数组 `nums` ，请你返回 **好的** 分割 `nums` 方案数目。由于答案可能会很大，请你将结果对 <code>10<sup>9</sup> + 7</code> 取余后返回。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,1,1]
<strong>输出:</strong> 1
<strong>解释:</strong> 唯一一种好的分割方案是将 nums 分成 [1] [1] [1] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,2,2,5,0]
<strong>输出:</strong> 3
<strong>解释:</strong> nums 总共有 3 种好的分割方案：
[1] [2] [2,2,5,0]
[1] [2,2] [2,5,0]
[1,2] [2,2] [5,0]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [3,2,1]
<strong>输出:</strong> 0
<strong>解释:</strong> 没有好的分割方案。
</pre>

#### 提示:
* <code>3 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 二分查找
```Rust
impl Solution {
    pub fn ways_to_split(mut nums: Vec<i32>) -> i32 {
        let mut ret = 0;

        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }

        let sum = *nums.last().unwrap();

        for i in 0..nums.len() - 2 {
            let j = match nums[i + 1..].binary_search(&(2 * nums[i] - 1)) {
                Ok(a) => a + 1,
                Err(b) => b,
            };
            let k = match nums[i + 1..].binary_search(&((sum - nums[i]) / 2 + nums[i])) {
                Ok(a) if a == nums.len() - i - 2 => a,
                Ok(a) => a + 1,
                Err(b) if b == nums.len() - i - 1 => b - 1,
                Err(b) => b,
            };

            ret = (ret + k.saturating_sub(j) as i32) % 1_000_000_007;
        }

        ret
    }
}
```
