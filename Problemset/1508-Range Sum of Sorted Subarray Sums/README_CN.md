# 1508. 子数组和排序后的区间和
给你一个数组 `nums` ，它包含 `n` 个正整数。你需要计算所有非空连续子数组的和，并将它们按升序排序，得到一个新的包含 `n * (n + 1) / 2` 个数字的数组。

请你返回在新数组中下标为 `left` 到 `right` （**下标从 1 开始**）的所有数字和（包括左右端点）。由于答案可能很大，请你将它对 10^9 + 7 取模后返回。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,4], n = 4, left = 1, right = 5
<strong>输出:</strong> 13
<strong>解释:</strong> 所有的子数组和为 1, 3, 6, 10, 2, 5, 9, 3, 7, 4 。将它们升序排序后，我们得到新的数组 [1, 2, 3, 3, 4, 5, 6, 7, 9, 10] 。下标从 le = 1 到 ri = 5 的和为 1 + 2 + 3 + 3 + 4 = 13 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,4], n = 4, left = 3, right = 4
<strong>输出:</strong> 6
<strong>解释:</strong> 给定数组与示例 1 一样，所以新数组为 [1, 2, 3, 3, 4, 5, 6, 7, 9, 10] 。下标从 le = 3 到 ri = 4 的和为 3 + 3 = 6 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,2,3,4], n = 4, left = 1, right = 10
<strong>输出:</strong> 50
</pre>

#### 提示:
* `1 <= nums.length <= 10^3`
* `nums.length == n`
* `1 <= nums[i] <= 100`
* `1 <= left <= right <= n * (n + 1) / 2`

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[]} nums
# @param {Integer} n
# @param {Integer} left
# @param {Integer} right
# @return {Integer}
def range_sum(nums, _n, left, right)
  sums = []
  ret = 0

  (0...nums.size).each do |i|
    sum = 0
    (i...nums.size).each do |j|
      sum += nums[j]
      sums.push(sum)
    end
  end

  sums.sort!

  (left..right).each do |i|
    ret = (ret + sums[i - 1]) % 1_000_000_007
  end

  ret
end
```

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let mut sums = vec![];
        let mut ret = 0;

        for i in 0..nums.len() {
            let mut sum = 0;
            for j in i..nums.len() {
                sum += nums[j];
                sums.push(sum);
            }
        }

        sums.sort_unstable();

        for i in left..=right {
            ret = (ret + sums[i as usize - 1]) % 1_000_000_007;
        }

        ret
    }
}
```
