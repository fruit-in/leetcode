# 2614. 对角线上的质数
给你一个下标从 **0** 开始的二维整数数组 `nums` 。

返回位于 `nums` 至少一条 **对角线** 上的最大 **质数** 。如果任一对角线上均不存在质数，返回 *0* 。

注意：
* 如果某个整数大于 `1` ，且不存在除 `1` 和自身之外的正整数因子，则认为该整数是一个质数。
* 如果存在整数 `i` ，使得 `nums[i][i] = val` 或者 `nums[i][nums.length - i - 1]= val` ，则认为整数 `val` 位于 `nums` 的一条对角线上。

![](https://assets.leetcode.com/uploads/2023/03/06/screenshot-2023-03-06-at-45648-pm.png)

在上图中，一条对角线是 **[1,5,9]** ，而另一条对角线是 **[3,5,7]** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [[1,2,3],[5,6,7],[9,10,11]]
<strong>输出:</strong> 11
<strong>解释:</strong> 数字 1、3、6、9 和 11 是所有 "位于至少一条对角线上" 的数字。由于 11 是最大的质数，故返回 11 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [[1,2,3],[5,17,7],[9,11,10]]
<strong>输出:</strong> 17
<strong>解释:</strong> 数字 1、3、9、10 和 17 是所有满足"位于至少一条对角线上"的数字。由于 17 是最大的质数，故返回 17 。
</pre>

#### 提示:
* `1 <= nums.length <= 300`
* <code>nums.length == nums<sub>i</sub>.length</code>
* <code>1 <= nums[i][j] <= 4*10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let mut diagonal_nums = (0..nums.len())
            .map(|i| [nums[i][i], nums[i][nums.len() - i - 1]])
            .flatten()
            .collect::<Vec<_>>();
        diagonal_nums.sort_unstable();

        *diagonal_nums
            .iter()
            .rev()
            .find(|&&num| {
                for x in 2..=(num as f64).sqrt() as i32 {
                    if num % x == 0 {
                        return false;
                    }
                }

                num > 1
            })
            .unwrap_or(&0)
    }
}
```
