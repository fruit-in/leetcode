# 494. 目标和
给你一个整数数组 `nums` 和一个整数 `target` 。

向数组中的每个整数前添加 `'+'` 或 `'-'` ，然后串联起所有整数，可以构造一个 **表达式** ：
* 例如，`nums = [2, 1]` ，可以在 `2` 之前添加 `'+'` ，在 `1` 之前添加 `'-'` ，然后串联起来得到表达式 `"+2-1"` 。

返回可以通过上述方法构造的、运算结果等于 `target` 的不同 **表达式** 的数目。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,1,1,1,1], target = 3
<strong>输出:</strong> 5
<strong>解释:</strong> 一共有 5 种方法让最终目标和为 3 。
-1 + 1 + 1 + 1 + 1 = 3
+1 - 1 + 1 + 1 + 1 = 3
+1 + 1 - 1 + 1 + 1 = 3
+1 + 1 + 1 - 1 + 1 = 3
+1 + 1 + 1 + 1 - 1 = 3
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1], target = 1
<strong>输出:</strong> 1
</pre>

#### 提示:
* `1 <= nums.length <= 20`
* `0 <= nums[i] <= 1000`
* `0 <= sum(nums[i]) <= 1000`
* `-1000 <= target <= 1000`

## 题解 (Ruby)

### 1. 动态规划
```Ruby
# @param {Integer[]} nums
# @param {Integer} target
# @return {Integer}
def find_target_sum_ways(nums, target)
  sum = nums.sum
  return 0 if sum < target.abs

  dp0 = [0] * (2 * sum + 1)
  dp0[sum] = 1

  nums.each do |num|
    dp1 = [0] * (2 * sum + 1)

    (0..2 * sum).each do |i|
      if dp0[i] > 0
        dp1[i + num] += dp0[i]
        dp1[i - num] += dp0[i]
      end
    end

    dp0 = dp1
  end

  dp0[sum + target]
end
```

## 题解 (Rust)

### 1. 动态规划
```Rust
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum = nums.iter().sum::<i32>() as usize;
        if sum < target.abs() as usize {
            return 0;
        }

        let mut dp0 = vec![0; 2 * sum + 1];
        dp0[sum] = 1;

        for num in nums {
            let mut dp1 = vec![0; 2 * sum + 1];

            for i in 0..2 * sum + 1 {
                if dp0[i] > 0 {
                    dp1[i + num as usize] += dp0[i];
                    dp1[i - num as usize] += dp0[i];
                }
            }

            dp0 = dp1;
        }

        dp0[(sum as i32 + target) as usize]
    }
}
```
