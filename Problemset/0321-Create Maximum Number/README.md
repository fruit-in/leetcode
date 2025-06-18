# 321. Create Maximum Number
You are given two integer arrays `nums1` and `nums2` of lengths `m` and `n` respectively. `nums1` and `nums2` represent the digits of two numbers. You are also given an integer `k`.

Create the maximum number of length `k <= m + n` from digits of the two numbers. The relative order of the digits from the same array must be preserved.

Return an array of the `k` digits representing the answer.

#### Example 1:
<pre>
<strong>Input:</strong> nums1 = [3,4,6,5], nums2 = [9,1,2,5,8,3], k = 5
<strong>Output:</strong> [9,8,6,5,3]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums1 = [6,7], nums2 = [6,0,4], k = 5
<strong>Output:</strong> [6,7,6,0,4]
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums1 = [3,9], nums2 = [8,9], k = 3
<strong>Output:</strong> [9,8,9]
</pre>

#### Constraints:
* `m == nums1.length`
* `n == nums2.length`
* `1 <= m, n <= 500`
* `0 <= nums1[i], nums2[i] <= 9`
* `1 <= k <= m + n`
* `nums1` and `nums2` do not have leading zeros.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let m = nums1.len();
        let n = nums2.len();
        let mut ret = vec![0; k];

        for k1 in 0.max(k.saturating_sub(n))..=m.min(k) {
            let k2 = k - k1;
            let mut stack1 = vec![];
            let mut stack2 = vec![];

            for i in 0..m {
                while m - i + stack1.len() > k1 && nums1[i] > *stack1.last().unwrap_or(&9) {
                    stack1.pop();
                }
                stack1.push(nums1[i]);
            }
            while stack1.len() > k1 {
                stack1.pop();
            }
            for i in 0..n {
                while n - i + stack2.len() > k2 && nums2[i] > *stack2.last().unwrap_or(&9) {
                    stack2.pop();
                }
                stack2.push(nums2[i]);
            }
            while stack2.len() > k2 {
                stack2.pop();
            }

            let mut i = 0;
            let mut j = 0;
            let mut tmp = vec![];
            let mut flag = false;

            stack1.push(-1);
            stack2.push(-2);

            while i < k1 || j < k2 {
                if j == k2 || stack1[i] > stack2[j] {
                    tmp.push(stack1[i]);
                    i += 1;
                } else if i == k1 || stack1[i] < stack2[j] {
                    tmp.push(stack2[j]);
                    j += 1;
                } else {
                    for l in 1..=(k1 - i).min(k2 - j) {
                        if stack1[i + l] > stack2[j + l] {
                            tmp.push(stack1[i]);
                            i += 1;
                            break;
                        } else if stack1[i + l] < stack2[j + l] {
                            tmp.push(stack2[j]);
                            j += 1;
                            break;
                        }
                    }
                }

                if tmp[i + j - 1] > ret[i + j - 1] {
                    flag = true;
                } else if !flag && tmp[i + j - 1] < ret[i + j - 1] {
                    break;
                }
            }

            if flag {
                ret = tmp;
            }
        }

        ret
    }
}
```
