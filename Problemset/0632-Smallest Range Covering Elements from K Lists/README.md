# 632. Smallest Range Covering Elements from K Lists
You have `k` lists of sorted integers in **non-decreasing order**. Find the **smallest** range that includes at least one number from each of the `k` lists.

We define the range `[a, b]` is smaller than range `[c, d]` if `b - a < d - c` **or** `a < c` if `b - a == d - c`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [[4,10,15,24,26],[0,9,12,20],[5,18,22,30]]
<strong>Output:</strong> [20,24]
<strong>Explanation:</strong>
List 1: [4, 10, 15, 24,26], 24 is in range [20,24].
List 2: [0, 9, 12, 20], 20 is in range [20,24].
List 3: [5, 18, 22, 30], 22 is in range [20,24].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [[1,2,3],[1,2,3],[1,2,3]]
<strong>Output:</strong> [1,1]
</pre>

#### Constraints:
* `nums.length == k`
* `1 <= k <= 3500`
* `1 <= nums[i].length <= 50`
* <code>-10<sup>5</sup> <= nums[i][j] <= 10<sup>5</sup></code>
* `nums[i]` is sorted in **non-decreasing** order.

## Solutions (Rust)

### 1. Solution
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
