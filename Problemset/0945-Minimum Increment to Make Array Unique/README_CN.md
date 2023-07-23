# 945. 使数组唯一的最小增量
给定整数数组 `A`，每次 *move* 操作将会选择任意 `A[i]`，并将其递增 `1`。

返回使 `A` 中的每个值都是唯一的最少操作次数。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,2,2]
<strong>输出:</strong> 1
<strong>解释:</strong> 经过一次 move 操作，数组将变为 [1, 2, 3]。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [3,2,1,2,1,7]
<strong>输出:</strong> 6
<strong>解释:</strong> 经过 6 次 move 操作，数组将变为 [3, 4, 1, 2, 5, 7]。
可以看出 5 次或 5 次以下的 move 操作是不能让数组的每个值唯一的。
</pre>

#### 提示:
1. `0 <= A.length <= 40000`
2. `0 <= A[i] < 40000`

## 题解 (Ruby)

### 1. 排序
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def min_increment_for_unique(nums)
  nums.sort!
  ret = 0

  (1...nums.size).each do |i|
    ret += [nums[i], nums[i - 1] + 1].max - nums[i]
    nums[i] = [nums[i], nums[i - 1] + 1].max
  end

  ret
end
```

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut ret = 0;

        for i in 1..nums.len() {
            ret += nums[i].max(nums[i - 1] + 1) - nums[i];
            nums[i] = nums[i].max(nums[i - 1] + 1);
        }

        ret
    }
}
```
