# 632. 最小区间
你有 `k` 个 **非递减排列** 的整数列表。找到一个 **最小** 区间，使得 `k` 个列表中的每个列表至少有一个数包含在其中。

我们定义如果 `b-a < d-c` 或者在 `b-a == d-c` 时 `a < c`，则区间 `[a,b]` 比 `[c,d]` 小。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [[4,10,15,24,26],[0,9,12,20],[5,18,22,30]]
<strong>输出:</strong> [20,24]
<strong>解释:</strong>
列表 1：[4, 10, 15, 24, 26]，24 在区间 [20,24] 中。
列表 2：[0, 9, 12, 20]，20 在区间 [20,24] 中。
列表 3：[5, 18, 22, 30]，22 在区间 [20,24] 中。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [[1,2,3],[1,2,3],[1,2,3]]
<strong>输出:</strong> [1,1]
</pre>

#### 提示:
* `nums.length == k`
* `1 <= k <= 3500`
* `1 <= nums[i].length <= 50`
* <code>-10<sup>5</sup> <= nums[i][j] <= 10<sup>5</sup></code>
* `nums[i]` 按非递减顺序排列

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let k = nums.len();
        let mut concat_nums = vec![];
        let mut subarray_counter = vec![0; k];
        let mut covered_subarrays = 0;
        let (mut a, mut b) = (-100000, 100000);

        for i in 0..k {
            for j in 0..nums[i].len() {
                concat_nums.push((nums[i][j], i));
            }
        }
        concat_nums.sort_unstable();

        let mut i = 0;

        for j in 0..concat_nums.len() {
            subarray_counter[concat_nums[j].1] += 1;
            if subarray_counter[concat_nums[j].1] == 1 {
                covered_subarrays += 1;
            }

            while covered_subarrays == k {
                let (c, d) = (concat_nums[i].0, concat_nums[j].0);
                if b - a > d - c || (b - a == d - c && a > c) {
                    (a, b) = (c, d);
                }

                subarray_counter[concat_nums[i].1] -= 1;
                if subarray_counter[concat_nums[i].1] == 0 {
                    covered_subarrays -= 1;
                }
                i += 1;
            }
        }

        vec![a, b]
    }
}
```
