# 506. 相对名次
给出 **N** 名运动员的成绩，找出他们的相对名次并授予前三名对应的奖牌。前三名运动员将会被分别授予 “金牌”，“银牌” 和“ 铜牌”（"Gold Medal", "Silver Medal", "Bronze Medal"）。

(注：分数越高的选手，排名越靠前。)

#### 示例 1:
<pre>
<strong>输入:</strong> [5, 4, 3, 2, 1]
<strong>输出:</strong> ["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
<strong>解释:</strong> 前三名运动员的成绩为前三高的，因此将会分别被授予 “金牌”，“银牌”和“铜牌” ("Gold Medal", "Silver Medal" and "Bronze Medal").
余下的两名运动员，我们只需要通过他们的成绩计算将其相对名次即可。
</pre>

#### 提示:
1. N 是一个正整数并且不会超过 10000。
2. 所有运动员的成绩都不相同。

## 题解 (Rust)

### 1. 哈希表
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_relative_ranks(nums: Vec<i32>) -> Vec<String> {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort_unstable_by(|a, b| b.cmp(a));
        let mut score_ranks = HashMap::new();

        for i in 0..sorted_nums.len() {
            match i {
                0 => score_ranks.insert(sorted_nums[i], "Gold Medal".to_string()),
                1 => score_ranks.insert(sorted_nums[i], "Silver Medal".to_string()),
                2 => score_ranks.insert(sorted_nums[i], "Bronze Medal".to_string()),
                _ => score_ranks.insert(sorted_nums[i], (i + 1).to_string()),
            };
        }

        nums.iter().map(|x| score_ranks.get(x).unwrap().to_string()).collect()
    }
}
```

### 2. 二分查找
```Rust
impl Solution {
    pub fn find_relative_ranks(nums: Vec<i32>) -> Vec<String> {
        let mut sorted_nums = nums.clone();
        sorted_nums.sort_unstable();
        let mut ranks = Vec::new();

        for score in nums {
            match sorted_nums.len() - sorted_nums.binary_search(&score).unwrap() - 1 {
                0 => ranks.push("Gold Medal".to_string()),
                1 => ranks.push("Silver Medal".to_string()),
                2 => ranks.push("Bronze Medal".to_string()),
                i => ranks.push((i + 1).to_string()),
            };
        }

        ranks
    }
}
```
