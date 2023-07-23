# 1630. Arithmetic Subarrays
A sequence of numbers is called **arithmetic** if it consists of at least two elements, and the difference between every two consecutive elements is the same. More formally, a sequence `s` is arithmetic if and only if `s[i+1] - s[i] == s[1] - s[0]` for all valid `i`.

For example, these are **arithmetic** sequences:
```
1, 3, 5, 7, 9
7, 7, 7, 7
3, -1, -5, -9
```

The following sequence is not **arithmetic**:
```
1, 1, 2, 5, 7
```

You are given an array of `n` integers, `nums`, and two arrays of `m` integers each, `l` and `r`, representing the `m` range queries, where the <code>i<sup>th</sup></code> query is the range `[l[i], r[i]]`. All the arrays are **0-indexed**.

Return *a list of* `boolean` *elements* `answer`*, where* `answer[i]` *is* `true` *if the subarray* `nums[l[i]], nums[l[i]+1], ... , nums[r[i]]` *can be **rearranged** to form an **arithmetic** sequence, and* `false` *otherwise*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [4,6,5,9,3,7], l = [0,0,2], r = [2,3,5]
<strong>Output:</strong> [true,false,true]
<strong>Explanation:</strong>
In the 0<sup>th</sup> query, the subarray is [4,6,5]. This can be rearranged as [6,5,4], which is an arithmetic sequence.
In the 1<sup>st</sup> query, the subarray is [4,6,5,9]. This cannot be rearranged as an arithmetic sequence.
In the 2<sup>nd</sup> query, the subarray is [5,9,3,7]. This can be rearranged as [3,5,7,9], which is an arithmetic sequence.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [-12,-9,-3,-12,-6,15,20,-25,-20,-15,-10], l = [0,1,6,4,8,7], r = [4,4,9,7,9,10]
<strong>Output:</strong> [false,true,false,false,true,true]
</pre>

#### Constraints:
* `n == nums.length`
* `m == l.length`
* `m == r.length`
* `2 <= n <= 500`
* `1 <= m <= 500`
* `0 <= l[i] < r[i] < n`
* <code>-10<sup>5</sup> <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Ruby)

### 1. Sort
```Ruby
# @param {Integer[]} nums
# @param {Integer[]} l
# @param {Integer[]} r
# @return {Boolean[]}
def check_arithmetic_subarrays(nums, l, r)
  ret = [false] * l.size

  (0...l.size).each do |i|
    sub = nums[l[i]..r[i]].sort
    ret[i] = sub.size > 1 && (2...sub.size).all? { |j| sub[j] - sub[j - 1] == sub[1] - sub[0] }
  end

  ret
end
```

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut ret = vec![false; l.len()];

        for i in 0..l.len() {
            let mut sub = nums[l[i] as usize..=r[i] as usize].to_vec();
            sub.sort_unstable();
            ret[i] =
                sub.len() > 1 && (2..sub.len()).all(|j| sub[j] - sub[j - 1] == sub[1] - sub[0]);
        }

        ret
    }
}
```
