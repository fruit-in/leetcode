# 1664. 生成平衡数组的方案数
给你一个整数数组 `nums` 。你需要选择 **恰好** 一个下标（下标从 **0** 开始）并删除对应的元素。请注意剩下元素的下标可能会因为删除操作而发生改变。

比方说，如果 `nums = [6,1,7,4,1]` ，那么：
* 选择删除下标 `1` ，剩下的数组为 `nums = [6,7,4,1]` 。
* 选择删除下标 `2` ，剩下的数组为 `nums = [6,1,4,1]` 。
* 选择删除下标 `4` ，剩下的数组为 `nums = [6,1,7,4]` 。

如果一个数组满足奇数下标元素的和与偶数下标元素的和相等，该数组就是一个 **平衡数组** 。

请你返回删除操作后，剩下的数组 `nums` 是 **平衡数组** 的 **方案数** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,1,6,4]
<strong>输出:</strong> 1
<strong>解释:</strong>
删除下标 0 ：[1,6,4] -> 偶数元素下标为：1 + 4 = 5 。奇数元素下标为：6 。不平衡。
删除下标 1 ：[2,6,4] -> 偶数元素下标为：2 + 4 = 6 。奇数元素下标为：6 。平衡。
删除下标 2 ：[2,1,4] -> 偶数元素下标为：2 + 4 = 6 。奇数元素下标为：1 。不平衡。
删除下标 3 ：[2,1,6] -> 偶数元素下标为：2 + 6 = 8 。奇数元素下标为：1 。不平衡。
只有一种让剩余数组成为平衡数组的方案。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,1,1]
<strong>输出:</strong> 3
<strong>解释:</strong> 你可以删除任意元素，剩余数组都是平衡数组。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,2,3]
<strong>输出:</strong> 0
<strong>解释:</strong> 不管删除哪个元素，剩下数组都不是平衡数组。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>4</sup></code>

## 题解 (Ruby)

### 1. 题解
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

## 题解 (Rust)

### 1. 题解
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
