# 506. Relative Ranks
Given scores of **N** athletes, find their relative ranks and the people with the top three highest scores, who will be awarded medals: "Gold Medal", "Silver Medal" and "Bronze Medal".

#### Example 1:
<pre>
<strong>Input:</strong> [5, 4, 3, 2, 1]
<strong>Output:</strong> ["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
<strong>Explanation:</strong> The first three athletes got the top three highest scores, so they got "Gold Medal", "Silver Medal" and "Bronze Medal". 
For the left two athletes, you just need to output their relative ranks according to their scores.
</pre>

#### Note:
1. N is a positive integer and won't exceed 10,000.
2. All the scores of athletes are guaranteed to be unique.

## Solutions (Rust)

### 1. HashMap
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

### 2. Binary Search
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
