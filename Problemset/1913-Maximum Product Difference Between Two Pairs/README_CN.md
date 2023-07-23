# 1913. 两个数对之间的最大乘积差
两个数对 `(a, b)` 和 `(c, d)` 之间的 **乘积差** 定义为 `(a * b) - (c * d)` 。
* 例如，`(5, 6)` 和 `(2, 7)` 之间的乘积差是 `(5 * 6) - (2 * 7) = 16` 。

给你一个整数数组 `nums` ，选出四个 **不同的** 下标 `w`、`x`、`y` 和 `z` ，使数对 `(nums[w], nums[x])` 和 `(nums[y], nums[z])` 之间的 **乘积差** 取到 **最大值** 。

返回以这种方式取得的乘积差中的 **最大值** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [5,6,2,7,4]
<strong>输出:</strong> 34
<strong>解释:</strong> 可以选出下标为 1 和 3 的元素构成第一个数对 (6, 7) 以及下标 2 和 4 构成第二个数对 (2, 4)
乘积差是 (6 * 7) - (2 * 4) = 34
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [4,2,5,9,7,4,8]
<strong>输出:</strong> 64
<strong>解释:</strong> 可以选出下标为 3 和 6 的元素构成第一个数对 (9, 8) 以及下标 1 和 5 构成第二个数对 (2, 4)
乘积差是 (9 * 8) - (2 * 4) = 64
</pre>

#### 提示:
* <code>4 <= nums.length <= 10<sup>4</sup></code>
* <code>1 <= nums[i] <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut max = (i32::MIN, i32::MIN);
        let mut min = (i32::MAX, i32::MAX);

        for num in nums {
            if num >= max.0 {
                max = (num, max.0);
            } else if num > max.1 {
                max.1 = num;
            }
            if num <= min.0 {
                min = (num, min.0)
            } else if num < min.1 {
                min.1 = num;
            }
        }

        max.0 * max.1 - min.0 * min.1
    }
}
```
