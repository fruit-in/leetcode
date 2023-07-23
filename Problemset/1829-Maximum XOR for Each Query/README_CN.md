# 1829. 每个查询的最大异或值
给你一个 **有序** 数组 `nums` ，它由 `n` 个非负整数组成，同时给你一个整数 `maximumBit` 。你需要执行以下查询 `n` 次：

1. 找到一个非负整数 <code>k < 2<sup>maximumBit</sup></code> ，使得 `nums[0] XOR nums[1] XOR ... XOR nums[nums.length-1] XOR k` 的结果 **最大化** 。`k` 是第 `i` 个查询的答案。
2. 从当前数组 `nums` 删除 **最后** 一个元素。

请你返回一个数组 `answer` ，其中 `answer[i]`是第 `i` 个查询的结果。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [0,1,1,3], maximumBit = 2
<strong>输出:</strong> [0,3,2,3]
<strong>解释:</strong> 查询的答案如下：
第一个查询：nums = [0,1,1,3]，k = 0，因为 0 XOR 1 XOR 1 XOR 3 XOR 0 = 3 。
第二个查询：nums = [0,1,1]，k = 3，因为 0 XOR 1 XOR 1 XOR 3 = 3 。
第三个查询：nums = [0,1]，k = 2，因为 0 XOR 1 XOR 2 = 3 。
第四个查询：nums = [0]，k = 3，因为 0 XOR 3 = 3 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,3,4,7], maximumBit = 3
<strong>输出:</strong> [5,2,6,5]
<strong>解释:</strong> 查询的答案如下：
第一个查询：nums = [2,3,4,7]，k = 5，因为 2 XOR 3 XOR 4 XOR 7 XOR 5 = 7。
第二个查询：nums = [2,3,4]，k = 2，因为 2 XOR 3 XOR 4 XOR 2 = 7 。
第三个查询：nums = [2,3]，k = 6，因为 2 XOR 3 XOR 6 = 7 。
第四个查询：nums = [2]，k = 5，因为 2 XOR 5 = 7 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [0,1,2,2,5,7], maximumBit = 3
<strong>输出:</strong> [4,3,6,4,6,7]
</pre>

#### 提示:
* `nums.length == n`
* <code>1 <= n <= 10<sup>5</sup></code>
* `1 <= maximumBit <= 20`
* <code>0 <= nums[i] < 2<sup>maximumBit</sup></code>
* `nums` 中的数字已经按 **升序** 排好序。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut xor = nums.iter().fold(0, |acc, x| acc ^ x);
        let mut answer = vec![(1 << maximum_bit) - 1; nums.len()];

        for i in 0..nums.len() {
            answer[i] ^= xor;
            xor ^= nums[nums.len() - 1 - i];
        }

        answer
    }
}
```
