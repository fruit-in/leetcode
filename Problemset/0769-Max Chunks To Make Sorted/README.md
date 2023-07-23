# 769. Max Chunks To Make Sorted
Given an array `arr` that is a permutation of `[0, 1, ..., arr.length - 1]`, we split the array into some number of "chunks" (partitions), and individually sort each chunk.  After concatenating them, the result equals the sorted array.

What is the most number of chunks we could have made?

#### Example 1:
<pre>
<strong>Input:</strong> arr = [4,3,2,1,0]
<strong>Output:</strong> 1
<strong>Explanation:</strong>
Splitting into two or more chunks will not return the required result.
For example, splitting into [4, 3], [2, 1, 0] will result in [3, 4, 0, 1, 2], which isn't sorted.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,0,2,3,4]
<strong>Output:</strong> 4
<strong>Explanation:</strong>
We can split into two chunks, such as [1, 0], [2, 3, 4].
However, splitting into [1, 0], [2], [3], [4] is the highest number of chunks possible.
</pre>

#### Note:
* `arr` will have length in range `[1, 10]`.
* `arr[i]` will be a permutation of `[0, 1, ..., arr.length - 1]`.

## Solutions (Ruby)

### 1. Solution
```Ruby
# @param {Integer[]} arr
# @return {Integer}
def max_chunks_to_sorted(arr)
  x = 0
  ret = 0

  (0...arr.size).each do |i|
    x ^= 1 << arr[i]
    ret += 1 if x == (2 << i) - 1
  end

  ret
end
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut x = 0;
        let mut ret = 0;

        for i in 0..arr.len() {
            x ^= 1 << arr[i];
            if x == (2 << i) - 1 {
                ret += 1;
            }
        }

        ret
    }
}
```
