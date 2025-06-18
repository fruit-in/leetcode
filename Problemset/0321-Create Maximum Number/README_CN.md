# 321. 拼接最大数
给你两个整数数组 `nums1` 和 `nums2`，它们的长度分别为 `m` 和 `n`。数组 `nums1` 和 `nums2` 分别代表两个数各位上的数字。同时你也会得到一个整数 `k`。

请你利用这两个数组中的数字创建一个长度为 `k <= m + n` 的最大数。同一数组中数字的相对顺序必须保持不变。

返回代表答案的长度为 `k` 的数组。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [3,4,6,5], nums2 = [9,1,2,5,8,3], k = 5
<strong>输出:</strong> [9,8,6,5,3]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [6,7], nums2 = [6,0,4], k = 5
<strong>输出:</strong> [6,7,6,0,4]
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums1 = [3,9], nums2 = [8,9], k = 3
<strong>输出:</strong> [9,8,9]
</pre>

#### 提示:
* `m == nums1.length`
* `n == nums2.length`
* `1 <= m, n <= 500`
* `0 <= nums1[i], nums2[i] <= 9`
* `1 <= k <= m + n`
* `nums1` 和 `nums2` 没有前导 0。

## 题解 (Rust)

### 1. 题解
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
