# 2523. Closest Prime Numbers in Range
Given two positive integers `left` and `right`, find the two integers `num1` and `num2` such that:

* `left <= nums1 < nums2 <= right`.
* `nums1` and `nums2` are both **prime** numbers.
* `nums2 - nums1` is the **minimum** amongst all other pairs satisfying the above conditions.

Return *the positive integer array* `ans = [nums1, nums2]`. *If there are multiple pairs satisfying these conditions, return the one with the minimum* `nums1` *value or* `[-1, -1]` *if such numbers do not exist*.

A number greater than `1` is called **prime** if it is only divisible by `1` and itself.

#### Example 1:
<pre>
<strong>Input:</strong> left = 10, right = 19
<strong>Output:</strong> [11,13]
<strong>Explanation:</strong> The prime numbers between 10 and 19 are 11, 13, 17, and 19.
The closest gap between any pair is 2, which can be achieved by [11,13] or [17,19].
Since 11 is smaller than 17, we return the first pair.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> left = 4, right = 6
<strong>Output:</strong> [-1,-1]
<strong>Explanation:</strong> There exists only one prime number in the given range, so the conditions cannot be satisfied.
</pre>

#### Constraints:
* <code>1 <= left <= right <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
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
