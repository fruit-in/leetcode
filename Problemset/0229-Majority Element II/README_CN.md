# 229. 多数元素 II
给定一个大小为 *n* 的整数数组，找出其中所有出现超过 `⌊ n/3 ⌋` 次的元素。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,2,3]
<strong>输出:</strong> [3]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1]
<strong>输出:</strong> [1]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,2]
<strong>输出:</strong> [1,2]
</pre>

#### 提示:
* <code>1 <= nums.length <= 5 * 10<sup>4</sup></code>
* <code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code>

**进阶：**尝试设计时间复杂度为 O(n)、空间复杂度为 O(1)的算法解决此问题。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut majorities = [(i32::MAX, 0); 2];

        for num in &nums {
            if let Some(i) = majorities.iter().position(|(x, _)| x == num) {
                majorities[i].1 += 1;
            } else if let Some(i) = majorities.iter().position(|&(_, c)| c == 0) {
                majorities[i] = (*num, 1);
            } else {
                for i in 0..2 {
                    majorities[i].1 -= 1;
                }
            }
        }

        for i in 0..2 {
            majorities[i].1 = 0;
        }

        for num in &nums {
            if let Some(i) = majorities.iter().position(|(x, _)| x == num) {
                majorities[i].1 += 1;
            }
        }

        majorities
            .into_iter()
            .filter(|&(x, c)| *c > n / 3)
            .map(|(x, _)| *x)
            .collect()
    }
}
```
