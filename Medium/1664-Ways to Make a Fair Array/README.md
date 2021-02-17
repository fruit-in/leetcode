# 1664. Ways to Make a Fair Array
You are given an integer array `nums`. You can choose **exactly one** index (**0-indexed**) and remove the element. Notice that the index of the elements may change after the removal.

For example, if `nums = [6,1,7,4,1]`:
* Choosing to remove index `1` results in `nums = [6,7,4,1]`.
* Choosing to remove index `2` results in `nums = [6,1,4,1]`.
* Choosing to remove index `4` results in `nums = [6,1,7,4]`.

An array is **fair** if the sum of the odd-indexed values equals the sum of the even-indexed values.

Return the ***number** of indices that you could choose such that after the removal*, `nums` *is **fair***.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,1,6,4]
<strong>Output:</strong> 1
<strong>Explanation:</strong>
Remove index 0: [1,6,4] -> Even sum: 1 + 4 = 5. Odd sum: 6. Not fair.
Remove index 1: [2,6,4] -> Even sum: 2 + 4 = 6. Odd sum: 6. Fair.
Remove index 2: [2,1,4] -> Even sum: 2 + 4 = 6. Odd sum: 1. Not fair.
Remove index 3: [2,1,6] -> Even sum: 2 + 6 = 8. Odd sum: 1. Not fair.
There is 1 index that you can remove to make nums fair.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,1,1]
<strong>Output:</strong> 3
<strong>Explanation:</strong> You can remove any index and the remaining array is fair.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,2,3]
<strong>Output:</strong> 0
<strong>Explanation:</strong> You cannot make a fair array after removing any index.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>4</sup></code>

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def ways_to_make_fair(nums)
  even_sums = [nums[0]]
  odd_sums = [0]
  ret = 0

  (1...nums.length).each do |i|
    if i.even?
      even_sums.push(nums[i] + even_sums[i - 1])
      odd_sums.push(odd_sums[i - 1])
    else
      even_sums.push(even_sums[i - 1])
      odd_sums.push(nums[i] + odd_sums[i - 1])
    end
  end

  (0...nums.length).each do |i|
    even_sum = even_sums[i] + odd_sums[-1] - odd_sums[i] - (i.even? ? nums[i] : 0)
    odd_sum = odd_sums[i] + even_sums[-1] - even_sums[i] - (i.even? ? 0 : nums[i])

    ret += 1 if even_sum == odd_sum
  end

  ret
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut even_sums = vec![nums[0]];
        let mut odd_sums = vec![0];
        let mut ret = 0;

        for i in 1..n {
            if i % 2 == 0 {
                even_sums.push(nums[i] + even_sums[i - 1]);
                odd_sums.push(odd_sums[i - 1]);
            } else {
                even_sums.push(even_sums[i - 1]);
                odd_sums.push(nums[i] + odd_sums[i - 1]);
            }
        }

        for i in 0..n {
            let even_sum = match i % 2 {
                0 => even_sums[i] - nums[i] + odd_sums[n - 1] - odd_sums[i],
                _ => even_sums[i] + odd_sums[n - 1] - odd_sums[i],
            };
            let odd_sum = match i % 2 {
                0 => odd_sums[i] + even_sums[n - 1] - even_sums[i],
                _ => odd_sums[i] - nums[i] + even_sums[n - 1] - even_sums[i],
            };

            ret += (even_sum == odd_sum) as i32;
        }

        ret
    }
}
```
