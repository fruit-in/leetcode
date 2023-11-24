# 1574. Shortest Subarray to be Removed to Make Array Sorted
Given an integer array `arr`, remove a subarray (can be empty) from `arr` such that the remaining elements in `arr` are **non-decreasing**.

Return *the length of the shortest subarray to remove*.

A **subarray** is a contiguous subsequence of the array.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [1,2,3,10,4,2,3,5]
<strong>Output:</strong> 3
<strong>Explanation:</strong> The shortest subarray we can remove is [10,4,2] of length 3. The remaining elements after that will be [1,2,3,3,5] which are sorted.
Another correct solution is to remove the subarray [3,10,4].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [5,4,3,2,1]
<strong>Output:</strong> 4
<strong>Explanation:</strong> Since the array is strictly decreasing, we can only keep a single element. Therefore we need to remove a subarray of length 4, either [5,4,3,2] or [4,3,2,1].
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> arr = [1,2,3]
<strong>Output:</strong> 0
<strong>Explanation:</strong> The array is already non-decreasing. We do not need to remove any elements.
</pre>

#### Constraints:
* <code>1 <= arr.length <= 10<sup>5</sup></code>
* <code>0 <= arr[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let mut arr = arr;
        let mut i = arr.len();
        let mut ret = arr.len() - 1;

        arr.insert(0, 0);

        while i > 0 && arr[i - 1] <= arr[i] {
            i -= 1;
        }

        if i == 0 {
            return 0;
        }

        for j in 0..arr.len() {
            if j > 0 && arr[j - 1] > arr[j] {
                break;
            }

            while i < arr.len() && arr[i] < arr[j] {
                i += 1;
            }

            ret = ret.min(i - j - 1);
        }

        ret as i32
    }
}
```
