# 1906. Minimum Absolute Difference Queries
The **minimum absolute difference** of an array `a` is defined as the **minimum value** of `|a[i] - a[j]|`, where `0 <= i < j < a.length` and `a[i] != a[j]`. If all elements of `a` are the **same**, the minimum absolute difference is `-1`.

* For example, the minimum absolute difference of the array `[5,2,3,7,2]` is `|2 - 3| = 1`. Note that it is not `0` because `a[i]` and `a[j]` must be different.

You are given an integer array `nums` and the array `queries` where <code>queries[i] = [l<sub>i</sub>, r<sub>i</sub>]</code>. For each query `i`, compute the **minimum absolute difference** of the **subarray** <code>nums[l<sub>i</sub>...r<sub>i</sub>]</code> containing the elements of `nums` between the **0-based** indices <code>l<sub>i</sub></code> and <code>r<sub>i</sub></code> (**inclusive**).

Return *an **array*** `ans` *where* `ans[i]` *is the answer to the* <code>i<sup>th</sup></code> *query*.

A **subarray** is a contiguous sequence of elements in an array.

The value of `|x|` is defined as:
* `x if x >= 0`.
* `-x if x < 0`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,3,4,8], queries = [[0,1],[1,2],[2,3],[0,3]]
<strong>Output:</strong> [2,1,4,1]
<strong>Explanation:</strong> The queries are processed as follows:
- queries[0] = [0,1]: The subarray is [1,3] and the minimum absolute difference is |1-3| = 2.
- queries[1] = [1,2]: The subarray is [3,4] and the minimum absolute difference is |3-4| = 1.
- queries[2] = [2,3]: The subarray is [4,8] and the minimum absolute difference is |4-8| = 4.
- queries[3] = [0,3]: The subarray is [1,3,4,8] and the minimum absolute difference is |3-4| = 1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [4,5,2,2,7,10], queries = [[2,3],[0,2],[0,5],[3,5]]
<strong>Output:</strong> [-1,1,1,3]
<strong>Explanation:</strong> The queries are processed as follows:
- queries[0] = [2,3]: The subarray is [2,2] and the minimum absolute difference is -1 because all the
  elements are the same.
- queries[1] = [0,2]: The subarray is [4,5,2] and the minimum absolute difference is |4-5| = 1.
- queries[2] = [0,5]: The subarray is [4,5,2,2,7,10] and the minimum absolute difference is |4-5| = 1.
- queries[3] = [3,5]: The subarray is [2,7,10] and the minimum absolute difference is |7-10| = 3.
</pre>

#### Constraints:
* <code>2 <= nums.length <= 10<sup>5</sup></code>
* `1 <= nums[i] <= 100`
* <code>1 <= queries.length <= 2 * 10<sup>4</sup></code>
* <code>0 <= l<sub>i</sub> < r<sub>i</sub> < nums.length</code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn min_difference(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let max_num = *nums.iter().max().unwrap() as usize;
        let mut prefix_count = vec![vec![0; max_num + 1]];
        let mut ans = vec![-1; queries.len()];

        for i in 0..nums.len() {
            let mut count = prefix_count[i].clone();
            count[nums[i] as usize] += 1;
            prefix_count.push(count);
        }

        for i in 0..queries.len() {
            let (l, r) = (queries[i][0] as usize, queries[i][1] as usize);
            let mut prev = 0;

            for j in 1..=max_num {
                if prefix_count[r + 1][j] - prefix_count[l][j] > 0 {
                    if prev > 0 && (ans[i] == -1 || j as i32 - prev < ans[i]) {
                        ans[i] = j as i32 - prev;
                    }
                    prev = j as i32;
                }
            }
        }

        ans
    }
}
```
