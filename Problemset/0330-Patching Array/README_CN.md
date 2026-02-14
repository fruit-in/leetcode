# 330. 按要求补齐数组
给定一个已排序的正整数数组 `nums` ，和一个正整数 `n` 。从 `[1, n]` 区间内选取任意个数字补充到 `nums` 中，使得 `[1, n]` 区间内的任何数字都可以用 `nums` 中某几个数字的和来表示。

请返回 *满足上述要求的最少需要补充的数字个数* 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,3], n = 6
<strong>输出:</strong> 1
<strong>解释:</strong>
根据 nums 里现有的组合 [1], [3], [1,3]，可以得出 1, 3, 4。
现在如果我们将 2 添加到 nums 中， 组合变为: [1], [2], [3], [1,3], [2,3], [1,2,3]。
其和可以表示数字 1, 2, 3, 4, 5, 6，能够覆盖 [1, 6] 区间里所有的数。
所以我们最少需要添加一个数字。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,5,10], n = 20
<strong>输出:</strong> 2
<strong>解释:</strong> 我们需要添加 [2,4]。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,2,2], n = 5
<strong>输出:</strong> 0
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* <code>1 <= nums[i] <= 10<sup>4</sup></code>
* `nums` 按 **升序排列**
* <code>1 <= n <= 2<sup>31</sup> - 1</code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let nums = nums.into_iter().map(|num| num as i64).collect::<Vec<_>>();
        let n = n as i64;
        let mut sum = 0;
        let mut ret = 0;

        for &num in &nums {
            while sum < num - 1 && sum < n {
                sum += sum + 1;
                ret += 1;
            }
            sum += num;

            if sum >= n {
                break;
            }
        }

        while sum < n {
            sum += sum + 1;
            ret += 1;
        }

        ret
    }
}
```
