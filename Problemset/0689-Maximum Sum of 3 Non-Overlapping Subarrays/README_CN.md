# 689. 三个无重叠子数组的最大和
给你一个整数数组 `nums` 和一个整数 `k` ，找出三个长度为 `k` 、互不重叠、且全部数字和最大的子数组，并返回这三个子数组。

以下标的数组形式返回结果，数组中的每一项分别指示每个子数组的起始位置（下标从 **0** 开始）。如果有多个结果，返回字典序最小的一个。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,1,2,6,7,5,1], k = 2
<strong>输出:</strong> [0,3,5]
<strong>解释:</strong> 子数组 [1, 2], [2, 6], [7, 5] 对应的起始下标为 [0, 3, 5]。
也可以取 [2, 1], 但是结果 [1, 3, 5] 在字典序上更小。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,1,2,1,2,1,2,1], k = 2
<strong>输出:</strong> [0,2,4]
</pre>

#### 提示:
* <code>1 <= nums.length <= 2 * 10<sup>4</sup></code>
* <code>1 <= nums[i] < 2<sup>16</sup></code>
* `1 <= k <= floor(nums.length / 3)`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut subarray_sum = vec![0; nums.len() + 1 - k];
        let mut suffix_max1 = vec![(0, 0); nums.len() + 2 - k];
        let mut suffix_max2 = vec![(0, 0, 0); nums.len() + 2 - k * 2];
        let mut suffix_max3 = vec![(0, 0, 0, 0); nums.len() + 2 - k * 3];

        subarray_sum[0] = (0..k).map(|i| nums[i]).sum();
        for i in 1..subarray_sum.len() {
            subarray_sum[i] = subarray_sum[i - 1] - nums[i - 1] + nums[i + k - 1];
        }

        for i in (0..suffix_max1.len() - 1).rev() {
            suffix_max1[i] = suffix_max1[i + 1];
            if subarray_sum[i] >= suffix_max1[i].0 {
                suffix_max1[i] = (subarray_sum[i], i);
            }

            if i < suffix_max2.len() - 1 {
                suffix_max2[i] = suffix_max2[i + 1];
                if subarray_sum[i] + suffix_max1[i + k].0 >= suffix_max2[i].0 {
                    suffix_max2[i] = (
                        subarray_sum[i] + suffix_max1[i + k].0,
                        i,
                        suffix_max1[i + k].1,
                    );
                }
            }

            if i < suffix_max3.len() - 1 {
                suffix_max3[i] = suffix_max3[i + 1];
                if subarray_sum[i] + suffix_max2[i + k].0 >= suffix_max3[i].0 {
                    suffix_max3[i] = (
                        subarray_sum[i] + suffix_max2[i + k].0,
                        i,
                        suffix_max2[i + k].1,
                        suffix_max2[i + k].2,
                    );
                }
            }
        }

        vec![
            suffix_max3[0].1 as i32,
            suffix_max3[0].2 as i32,
            suffix_max3[0].3 as i32,
        ]
    }
}
```
