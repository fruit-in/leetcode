# 2523. 范围内最接近的两个质数
给你两个正整数 `left` 和 `right` ，请你找到两个整数 `num1` 和 `num2` ，它们满足：

* `left <= nums1 < nums2 <= right`  。
* `nums1` 和 `nums2` 都是 **质数** 。
* `nums2 - nums1` 是满足上述条件的质数对中的 **最小值** 。

请你返回正整数数组 `ans = [nums1, nums2]` 。如果有多个整数对满足上述条件，请你返回 `nums1` 最小的质数对。如果不存在符合题意的质数对，请你返回 `[-1, -1]` 。

如果一个整数大于 `1` ，且只能被 `1` 和它自己整除，那么它是一个质数。

#### 示例 1:
<pre>
<strong>输入:</strong> left = 10, right = 19
<strong>输出:</strong> [11,13]
<strong>解释:</strong> 10 到 19 之间的质数为 11 ，13 ，17 和 19 。
质数对的最小差值是 2 ，[11,13] 和 [17,19] 都可以得到最小差值。
由于 11 比 17 小，我们返回第一个质数对。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> left = 4, right = 6
<strong>输出:</strong> [-1,-1]
<strong>解释:</strong> 给定范围内只有一个质数，所以题目条件无法被满足。
</pre>

#### 提示:
* <code>1 <= left <= right <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut is_prime = vec![true; right as usize + 1];
        let mut primes = vec![];
        let mut ret = vec![-1, i32::MAX - 1];

        for nums2 in 2..=right {
            if ret[1] - ret[0] < 3 {
                break;
            }

            if is_prime[nums2 as usize] {
                if let Some(&nums1) = primes.last() {
                    if nums1 >= left && nums2 - nums1 < ret[1] - ret[0] {
                        ret = vec![nums1, nums2];
                    }
                }

                primes.push(nums2);
            }

            for prime in &primes {
                if prime * nums2 > right {
                    break;
                }

                is_prime[(prime * nums2) as usize] = false;

                if nums2 % prime == 0 {
                    break;
                }
            }
        }

        if ret[0] == -1 {
            ret[1] = -1;
        }

        ret
    }
}
```
