# 565. Array Nesting
A zero-indexed array A of length N contains all integers from 0 to N-1. Find and return the longest length of set S, where S[i] = {A[i], A[A[i]], A[A[A[i]]], ... } subjected to the rule below.

Suppose the first element in S starts with the selection of element A[i] of index = i, the next element in S should be A[A[i]], and then A[A[A[i]]]… By that analogy, we stop adding right before a duplicate element occurs in S.

#### Example 1:
<pre>
<strong>Input:</strong> A = [5,4,0,3,1,6,2]
<strong>Output:</strong> 4
<strong>Explanation:</strong>
A[0] = 5, A[1] = 4, A[2] = 0, A[3] = 3, A[4] = 1, A[5] = 6, A[6] = 2.

One of the longest S[K]:
S[0] = {A[0], A[5], A[6], A[2]} = {5, 6, 2, 0}
</pre>

#### Note:
1. N is an integer within the range [1, 20,000].
2. The elements of A are all distinct.
3. Each element of A is an integer within the range [0, N-1].

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[]} nums
# @return {Integer}
def array_nesting(nums)
  ret = 1

  (0...nums.size).each do |i|
    next if nums[i] < 0

    length = 0
    j = i
    while nums[j] >= 0
      nums[j] = -nums[j] - 1
      length += 1
      j = -nums[j] - 1
    end

    ret = [ret, length].max
  end

  ret
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn array_nesting(mut nums: Vec<i32>) -> i32 {
        let mut ret = 1;

        for i in 0..nums.len() {
            if nums[i] < 0 {
                continue;
            }

            let mut length = 0;
            let mut j = i;
            while nums[j] >= 0 {
                nums[j] = -nums[j] - 1;
                length += 1;
                j = (-nums[j] - 1) as usize;
            }

            ret = ret.max(length);
        }

        ret
    }
}
```
