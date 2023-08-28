# 2521. 数组乘积中的不同质因数数目
给你一个正整数数组 `nums` ，对 `nums` 所有元素求积之后，找出并返回乘积中 **不同质因数** 的数目。

**注意：**

* **质数** 是指大于 1 且仅能被 1 及自身整除的数字。
* 如果 `val2 / val1` 是一个整数，则整数 `val1` 是另一个整数 `val2` 的一个因数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,4,3,7,10,6]
<strong>输出:</strong> 4
<strong>解释:</strong>
nums 中所有元素的乘积是：2 * 4 * 3 * 7 * 10 * 6 = 10080 = 2<sup>5</sup> * 3<sup>2</sup> * 5 * 7 。
共有 4 个不同的质因数，所以返回 4 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,4,8,16]
<strong>输出:</strong> 1
<strong>解释:</strong>
nums 中所有元素的乘积是：2 * 4 * 8 * 16 = 1024 = 2<sup>10</sup> 。
共有 1 个不同的质因数，所以返回 1 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>4</sup></code>
* `2 <= nums[i] <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn distinct_prime_factors(nums: Vec<i32>) -> i32 {
        let max_num = *nums.iter().max().unwrap();
        let mut primes = vec![];

        for i in 2..=max_num {
            if (2..=(i as f64).sqrt() as i32).all(|j| i % j != 0) {
                primes.push(i);
            }
        }

        for i in 0..nums.len() {
            for j in 0..primes.len() {
                if nums[i] < primes[j] {
                    break;
                }

                if nums[i] % primes[j] == 0 {
                    primes[j] = 1;
                }
            }
        }

        primes.iter().filter(|&&x| x == 1).count() as i32
    }
}
```
