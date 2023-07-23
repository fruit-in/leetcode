# 90. Subsets II
Given a collection of integers that might contain duplicates, ***nums***, return all possible subsets (the power set).

**Note:** The solution set must not contain duplicate subsets.

#### Example:
<pre>
<strong>Input:</strong> [1,2,2]
<strong>Output:</strong>
[
  [2],
  [1],
  [1,2,2],
  [2,2],
  [1,2],
  []
]
</pre>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut cnt = 1;
        let mut ret = vec![vec![]];

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                cnt += 1;
            } else {
                cnt = 1;
            }

            let mut temp = ret.split_at(ret.len() - ret.len() / cnt).1.to_vec();
            temp.iter_mut().for_each(|x| x.push(nums[i]));
            ret.append(&mut temp);
        }

        ret
    }
}
```
