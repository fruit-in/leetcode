# 1486. 数组异或操作
给你两个整数，`n` 和 `start` 。

数组 `nums` 定义为：`nums[i] = start + 2*i`（下标从 0 开始）且 `n == nums.length` 。

请返回 `nums` 中所有元素按位异或（**XOR**）后得到的结果。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 5, start = 0
<strong>输出:</strong> 8
<strong>解释:</strong> 数组 nums 为 [0, 2, 4, 6, 8]，其中 (0 ^ 2 ^ 4 ^ 6 ^ 8) = 8 。
     "^" 为按位异或 XOR 运算符。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 4, start = 3
<strong>输出:</strong> 8
<strong>解释:</strong> 数组 nums 为 [3, 5, 7, 9]，其中 (3 ^ 5 ^ 7 ^ 9) = 8.
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 1, start = 7
<strong>输出:</strong> 7
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> n = 10, start = 5
<strong>输出:</strong> 2
</pre>

#### 提示:
* `1 <= n <= 1000`
* `0 <= start <= 1000`
* `n == nums.length`

## 题解 (Ruby)

### 1. 模拟
```Ruby
# @param {Integer} n
# @param {Integer} start
# @return {Integer}
def xor_operation(n, start)
    ret = 0

    (0...n).each do |i|
        ret ^= start + 2 * i
    end

    return ret
end
```

## Solutions (Rust)

### 1. 模拟
```Rust
impl Solution {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        (0..n).fold(0, |acc, i| acc ^ (start + 2 * i))
    }
}
```
