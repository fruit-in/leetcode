# 1508. Range Sum of Sorted Subarray Sums
Given the array `nums` consisting of `n` positive integers. You computed the sum of all non-empty continous subarrays from the array and then sort them in non-decreasing order, creating a new array of `n * (n + 1) / 2` numbers.

*Return the sum of the numbers from index* `left` *to index* `right` (**indexed from 1**), *inclusive, in the new array*. Since the answer can be a huge number return it modulo 10^9 + 7.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3,4], n = 4, left = 1, right = 5
<strong>Output:</strong> 13
<strong>Explanation:</strong> All subarray sums are 1, 3, 6, 10, 2, 5, 9, 3, 7, 4. After sorting them in non-decreasing order we have the new array [1, 2, 3, 3, 4, 5, 6, 7, 9, 10]. The sum of the numbers from index le = 1 to ri = 5 is 1 + 2 + 3 + 3 + 4 = 13.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,2,3,4], n = 4, left = 3, right = 4
<strong>Output:</strong> 6
<strong>Explanation:</strong> The given array is the same as example 1. We have the new array [1, 2, 3, 3, 4, 5, 6, 7, 9, 10]. The sum of the numbers from index le = 3 to ri = 4 is 3 + 3 = 6.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,2,3,4], n = 4, left = 1, right = 10
<strong>Output:</strong> 50
</pre>

#### Constraints:
* `1 <= nums.length <= 10^3`
* `nums.length == n`
* `1 <= nums[i] <= 100`
* `1 <= left <= right <= n * (n + 1) / 2`

## Solutions (Ruby)

### 1. Solution
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

## Solutions (Rust)

### 1. Solution
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
