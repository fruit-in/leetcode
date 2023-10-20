# 164. 最大间距
给定一个无序的数组 `nums`，返回 *数组在排序之后，相邻元素之间最大的差值* 。如果数组元素个数小于 2，则返回 `0` 。

您必须编写一个在「线性时间」内运行并使用「线性额外空间」的算法。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,6,9,1]
<strong>输出:</strong> 3
<strong>解释:</strong> 排序后的数组是 [1,3,6,9], 其中相邻元素 (3,6) 和 (6,9) 之间都存在最大差值 3。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [10]
<strong>输出:</strong> 0
<strong>解释:</strong> 数组元素个数小于 2，因此返回 0。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let mut nums0 = nums.into_iter().map(|x| x as usize).collect::<Vec<_>>();
        let mut x = 0xff;

        for i in (0..32).step_by(8) {
            let mut count = vec![0; 256];
            let mut nums1 = vec![0; nums0.len()];

            for j in 0..nums0.len() {
                count[(nums0[j] & x) >> i] += 1;
            }

            for j in 1..count.len() {
                count[j] += count[j - 1];
            }

            for j in (0..nums0.len()).rev() {
                count[(nums0[j] & x) >> i] -= 1;
                nums1[count[(nums0[j] & x) >> i]] = nums0[j];
            }

            nums0 = nums1;
            x <<= 8;
        }

        (1..nums0.len())
            .map(|i| nums0[i] - nums0[i - 1])
            .max()
            .unwrap_or(0) as i32
    }
}
```
