# 915. Partition Array into Disjoint Intervals
Given an array `A`, partition it into two (contiguous) subarrays `left` and `right` so that:
* Every element in `left` is less than or equal to every element in `right`.
* `left` and `right` are non-empty.
* `left` has the smallest possible size.

Return the **length** of `left` after such a partitioning.  It is guaranteed that such a partitioning exists.

#### Example 1:
<pre>
<strong>Input:</strong> [5,0,3,8,6]
<strong>Output:</strong> 3
<strong>Explanation:</strong> left = [5,0,3], right = [8,6]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [1,1,1,0,6,12]
<strong>Output:</strong> 4
<strong>Explanation:</strong> left = [1,1,1,0], right = [6,12]
</pre>

#### Note:
1. `2 <= A.length <= 30000`
2. `0 <= A[i] <= 10^6`
3. It is guaranteed there is at least one way to partition `A` as described.

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[]} a
# @return {Integer}
def partition_disjoint(a)
  max_left = a[0]
  max = a[0]
  length = 1

  (1...a.length).each do |i|
    if a[i] < max_left
      length = i + 1
      max_left = max
    elsif a[i] > max
      max = a[i]
    end
  end

  length
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn partition_disjoint(a: Vec<i32>) -> i32 {
        let mut max_left = a[0];
        let mut max = a[0];
        let mut length = 1;

        for i in 1..a.len() {
            if a[i] < max_left {
                length = i + 1;
                max_left = max;
            } else if a[i] > max {
                max = a[i];
            }
        }

        length as i32
    }
}
```
