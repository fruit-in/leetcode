# 368. 最大整除子集
给你一个由 无重复 正整数组成的集合 `nums` ，请你找出并返回其中最大的整除子集 `answer` ，子集中每一元素对 `(answer[i], answer[j])` 都应当满足：

* `answer[i] % answer[j] == 0` ，或
* `answer[j] % answer[i] == 0`

如果存在多个有效解子集，返回其中任何一个均可。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3]
<strong>输出:</strong> [1,2]
<strong>解释:</strong> [1,3] 也会被视为正确答案。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,4,8]
<strong>输出:</strong> [1,2,4,8]
</pre>

#### Constraints:
* `1 <= nums.length <= 1000`
* <code>1 <= nums[i] <= 2 * 10<sup>9</sup></code>
* `nums` 中的所有整数 **互不相同**

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut max_len = vec![1; nums.len()];
        let mut prev_index = vec![None; nums.len()];
        let mut answer = vec![];

        nums.sort_unstable();

        for i in 1..nums.len() {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && max_len[i] < max_len[j] + 1 {
                    max_len[i] = max_len[j] + 1;
                    prev_index[i] = Some(j);
                }
            }
        }

        let mut curr = (0..nums.len()).max_by_key(|&i| max_len[i]).unwrap();
        answer.push(nums[curr]);

        while let Some(i) = prev_index[curr] {
            curr = i;
            answer.push(nums[curr]);
        }

        answer
    }
}
```
