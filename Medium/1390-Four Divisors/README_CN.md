# 1390. 四因数
给你一个整数数组 `nums`，请你返回该数组中恰有四个因数的这些整数的各因数之和。如果数组中不存在满足题意的整数，则返回 `0` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [21,4,7]
<strong>输出:</strong> 32
<strong>解释:</strong>
21 有 4 个因数：1, 3, 7, 21
4 有 3 个因数：1, 2, 4
7 有 2 个因数：1, 7
答案仅为 21 的所有因数的和。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [21,21]
<strong>输出:</strong> 64
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,2,3,4,5]
<strong>输出:</strong> 0
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>4</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut ret = 0;

        for &x in &nums {
            let mut tmp = 0;

            for y in 2..=(x as f64).sqrt() as i32 {
                if x % y == 0 {
                    if tmp > 0 || x == y * y {
                        tmp = 0;
                        break;
                    }
                    tmp += y + x / y;
                }
            }

            if tmp > 0 {
                ret += 1 + x + tmp;
            }
        }

        ret
    }
}
```
