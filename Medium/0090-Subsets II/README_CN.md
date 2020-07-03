# 90. 子集 II
给定一个可能包含重复元素的整数数组 ***nums***，返回该数组所有可能的子集（幂集）。

**说明:** 解集不能包含重复的子集。

#### 示例:
<pre>
<strong>输入:</strong> [1,2,2]
<strong>输出:</strong>
[
  [2],
  [1],
  [1,2,2],
  [2,2],
  [1,2],
  []
]
</pre>

## 题解 (Rust)

### 1. 题解
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
